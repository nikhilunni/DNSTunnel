use std::cmp::min;

use nom::error::ParseError;

use crate::{DnsPacket, Question, STANDARD_REQUEST_HEADER};
use crate::bitmasks::{QueryClass, RecordType};

const MAX_Q_NAME_LEN: usize = 255;
const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz0123456789";

fn max_bytes_per_packet(domain_name: &str) -> usize {
    let domain_segments = domain_name.split('.');
    let q_name_suffix_len = domain_segments
        .map(|label| {
            label.len() + 1
        })
        .sum::<usize>();

    // 255 - domain data - null byte - encoded data len byte
    MAX_Q_NAME_LEN - q_name_suffix_len - 2
}

pub fn encode_data_as_dns_request(
    bytes: &[u8],
    domain_name: &str,
) -> Vec<DnsPacket> {
    let max_bytes_per_packet = max_bytes_per_packet(domain_name);

    if max_bytes_per_packet <= 0 {
        panic!("Domain name {} too long", domain_name);
    }

    let bytes = data_encoding::BASE32_DNSSEC.encode(bytes);

    let num_packets = if (bytes.len() % max_bytes_per_packet) == 0 {
        bytes.len() / max_bytes_per_packet
    } else {
        (bytes.len() / max_bytes_per_packet) + 1
    };

    let mut packets = vec![];

    for i in 0..num_packets {
        let start = i * max_bytes_per_packet;
        let end = min(bytes.len(), (i + 1) * max_bytes_per_packet);

        let mut q_name = String::new();
        q_name.push_str(&bytes[start..end]);
        q_name.push('.');
        q_name.push_str(domain_name);

        let question = Question {
            q_name,
            q_type: RecordType::Null.bits(),
            q_class: QueryClass::Internet.bits(),
        };

        let packet = DnsPacket {
            header: STANDARD_REQUEST_HEADER,
            question,
            answers: vec![],
        };

        packets.push(packet);
    }

    packets
}

pub fn decode_dns_request_as_data(
    packets: Vec<DnsPacket>,
    domain_name: &str,
) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    for packet in packets {
        let len = packet.question.q_name.len();
        out.extend_from_slice(&packet.question.q_name[0 .. len - domain_name.len() - 1].as_bytes());
        // out.push_str(&packet.question.q_name[0 .. len - domain_name.len() - 1]);
    }

    data_encoding::BASE32_DNSSEC.decode(&out).unwrap()
}

#[cfg(test)]
mod tests {
    use rand::RngCore;

    use crate::codec::{decode_dns_request_as_data, encode_data_as_dns_request};

    #[test]
    fn codec_small() {
        let data = b"Hello, world";
        test_encode_decode(data, "example.com");
    }

    #[test]
    fn codec_large() {
        let mut data = [0u8; 10_000];
        rand::thread_rng().fill_bytes(&mut data);
        test_encode_decode(&data, "example.com");
    }

    fn test_encode_decode(data: &[u8], domain_name: &str) {
        let packets = encode_data_as_dns_request(
            data,
            domain_name,
        );

        let result: Vec<u8> = decode_dns_request_as_data(packets, domain_name);

        assert_eq!(
            Vec::from(data),
            result
        )
    }
}
