use std::io;

use byteorder::{BigEndian, WriteBytesExt};

use crate::DnsPacket;

pub fn serialize_dns_packet(packet: &DnsPacket) -> io::Result<Vec<u8>> {
    let mut buffer = Vec::new();

    buffer.write_u16::<BigEndian>(packet.header.id)?;
    buffer.write_u16::<BigEndian>(packet.header.flags)?;
    buffer.write_u16::<BigEndian>(packet.header.qd_count)?;
    buffer.write_u16::<BigEndian>(packet.header.an_count)?;
    buffer.write_u16::<BigEndian>(packet.header.ns_count)?;
    buffer.write_u16::<BigEndian>(packet.header.ar_count)?;

    buffer.extend_from_slice(&serialize_domain_name(&packet.question.q_name));
    buffer.write_u8(0).unwrap();
    buffer.write_u16::<BigEndian>(packet.question.q_type)?;
    buffer.write_u16::<BigEndian>(packet.question.q_class)?;

    Ok(buffer)
}

pub fn serialize_domain_name(domain_name: &str) -> Vec<u8> {
    let mut buffer = Vec::new();

    domain_name
        .split('.')
        .for_each(|label| {
            buffer.write_u8(label.len() as u8).unwrap();
            buffer.extend_from_slice(label.as_bytes());
        });

    buffer.write_u8(0).unwrap();

    buffer
}
