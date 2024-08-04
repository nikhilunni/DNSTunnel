use crate::bitmasks::{QueryClass, RecordType, STANDARD_REQUEST_FLAGS};

pub mod serializer;
pub mod deserializer;
pub mod bitmasks;
pub mod codec;

#[derive(Debug, Eq, PartialEq)]
pub struct DnsPacket {
    pub header: Header,
    pub question: Question,
    pub answers: Vec<Answer>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Header {
    pub id: u16,
    pub flags: u16,
    pub qd_count: u16,
    pub an_count: u16,
    pub ns_count: u16,
    pub ar_count: u16
}

#[derive(Debug, Eq, PartialEq)]
pub struct Question {
    pub q_name: String,
    pub q_type: u16,
    pub q_class: u16
}

#[derive(Debug, Eq, PartialEq)]
pub struct Answer {
    name: String,
    q_type: u16,
    class: u16,
    ttl: u32,
    rd_length: u16,
    rdata: Vec<u8>
}

const STANDARD_REQUEST_HEADER: Header = Header {
    id: 1,
    flags: STANDARD_REQUEST_FLAGS,
    qd_count: 1,
    an_count: 0,
    ns_count: 0,
    ar_count: 0
};

pub fn get_dns_request_packet(
    url: &'static str,
    record_type: &RecordType,
) -> DnsPacket {
    let question = Question {
        q_name: String::from(url),
        q_type: record_type.bits(),
        q_class: QueryClass::Internet.bits(),
    };

    DnsPacket { header: STANDARD_REQUEST_HEADER, question, answers: vec![] }
}
