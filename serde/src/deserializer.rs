use nom::bytes::streaming as bytes_streaming;
use nom::IResult;
use nom::multi::many_m_n;
use nom::number::streaming as number_streaming;

use crate::{Answer, Header, DnsPacket, Question};

pub fn deserialize_packet(input: &[u8]) -> IResult<&[u8], DnsPacket> {
    let (input, header) = deserialize_header(input)?;
    let (input, question) = deserialize_question(input)?;
    let (input, answers) = deserialize_answers(input, header.an_count)?;

    Ok((
        input,
        DnsPacket {
            header,
            question,
            answers,
        }
    ))
}

fn deserialize_header(input: &[u8]) -> IResult<&[u8], Header> {
    let (input, id) = number_streaming::be_u16(input)?;
    let (input, flags) = number_streaming::be_u16(input)?;
    let (input, qd_count) = number_streaming::be_u16(input)?;
    let (input, an_count) = number_streaming::be_u16(input)?;
    let (input, ns_count) = number_streaming::be_u16(input)?;
    let (input, ar_count) = number_streaming::be_u16(input)?;

    Ok((
        input,
        Header {
            id,
            flags,
            qd_count,
            an_count,
            ns_count,
            ar_count,
        }
    ))
}

fn deserialize_question(input: &[u8]) -> IResult<&[u8], Question> {
    let (input, q_name) = deserialize_qname(input)?;
    let (input, q_type) = number_streaming::be_u16(input)?;
    let (input, q_class) = number_streaming::be_u16(input)?;

    Ok((
        input,
        Question {
            q_name,
            q_type,
            q_class,
        }
    ))
}

fn deserialize_qname(input: &[u8]) -> IResult<&[u8], String> {
    let mut q_name = String::new();

    let mut input = input;

    while input[0] != 0 {
        let (inner_input, label_len) = number_streaming::be_u8(input)?;
        let (inner_input, label) = bytes_streaming::take(label_len)(inner_input)?;

        q_name.push_str(std::str::from_utf8(label).unwrap());

        if inner_input[0] != 0 {
            q_name.push('.');
        }

        input = inner_input
    }

    let (input, _) = bytes_streaming::take(1usize)(input)?; // Consume the null byte

    Ok((input, q_name))
}

fn deserialize_answers(
    input: &[u8],
    num_answers: u16,
) -> IResult<&[u8], Vec<Answer>> {
    many_m_n(num_answers as usize, num_answers as usize, deserialize_answer)(input)
}

fn deserialize_answer(input: &[u8]) -> IResult<&[u8], Answer> {

    let len = input[0];

    // Handle optional q_name compression
    let (input, name) = match len & 0xC0 {
        0xC0 => {
            let (input, _) = bytes_streaming::take(2usize)(input)?;
            (input, String::from("compressed"))
        },
        0x00 => deserialize_qname(input)?,
        _ => panic!("Invalid length")
    };

    let (input, q_type) = number_streaming::be_u16(input)?;
    let (input, class) = number_streaming::be_u16(input)?;
    let (input, ttl) = number_streaming::be_u32(input)?;
    let (input, rd_length) = number_streaming::be_u16(input)?;
    let (input, rdata) = bytes_streaming::take(rd_length)(input)?;

    Ok((
        input,
        Answer {
            name,
            q_type,
            class,
            ttl,
            rd_length,
            rdata: rdata.to_vec(),
        }
    ))
}

#[cfg(test)]
mod tests {
    use crate::{Header, DnsPacket, Question, Answer};
    use crate::bitmasks::{QueryClass, RecordType};

    #[test]
    fn should_deserialize_a_datagram() {
        let input = [
            0x00, 0x01, 0x81, 0x80, 0x00, 0x01, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x06, 0x63, 0x6f, 0x72,
            0x74, 0x65, 0x78, 0x02, 0x69, 0x6f, 0x00, 0x00, 0x01, 0x00, 0x01, 0xc0, 0x0c, 0x00, 0x01, 0x00,
            0x01, 0x00, 0x00, 0x38, 0x40, 0x00, 0x04, 0x4b, 0x02, 0x46, 0x4b, 0xc0, 0x0c, 0x00, 0x01, 0x00,
            0x01, 0x00, 0x00, 0x38, 0x40, 0x00, 0x04, 0x63, 0x53, 0xbe, 0x66
        ];

        let result = super::deserialize_packet(&input);

        assert_eq!(result.is_ok(), true);

        let (_, packet) = result.unwrap();

        assert_eq!(
            packet,
            DnsPacket {
                header: Header {
                    id: 1,
                    flags: 0x8180,
                    qd_count: 1,
                    an_count: 2,
                    ns_count: 0,
                    ar_count: 0
                },
                question: Question {
                    q_name: "cortex.io".to_string(),
                    q_type: RecordType::Null.bits(),
                    q_class: QueryClass::Internet.bits(),
                },
                answers: vec![
                    Answer {
                        name: "compressed".to_string(),
                        q_type: 1,
                        class: 1,
                        ttl: 14400,
                        rd_length: 4,
                        rdata: vec![0x4b, 0x02, 0x46, 0x4b]
                    },
                    Answer {
                        name: "compressed".to_string(),
                        q_type: 1,
                        class: 1,
                        ttl: 14400,
                        rd_length: 4,
                        rdata: vec![0x63, 0x53, 0xbe, 0x66]
                    }
                ]
            }
        )
    }
}
