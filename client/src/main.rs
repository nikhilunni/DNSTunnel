use std::net::UdpSocket;

use clap::Parser;

mod resolvconf;
mod tunnel;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Reserved domain name to use for queries
    #[arg(short, long)]
    domain_name: String,

    /// Optional nameserver to query
    #[arg(short, long, value_name = "ns")]
    name_server: Option<String>,
}


fn main() {
    println!("Hello, world!");
}

pub fn send_data(
    data: &[u8],
    socket: &UdpSocket,
) -> std::io::Result<()> {
    todo!()
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::net::UdpSocket;
    use serde::codec::{encode_data_as_dns_request};

    use serde::deserializer::deserialize_packet;
    use serde::DnsPacket;
    use serde::serializer::serialize_dns_packet;

    use crate::resolvconf;

    #[test]
    fn it_works() -> io::Result<()> {
        let server = resolvconf::get_resolvconf_addr()?;
        let socket = UdpSocket::bind("0.0.0.0:0")?;

        let packets = encode_data_as_dns_request(
            b"hello",
            "2vb.in",
        );

        for packet in packets {
            let serialized_packet = serialize_dns_packet(&packet)?;
            println!("Send request: {:x?}", serialized_packet);
            socket.send_to(&serialized_packet, &server)?;

            let response = get_response(&socket).unwrap();
            println!("Received response: {:x?}", response);
        }

        Ok(())
    }

    fn get_response(socket: &UdpSocket) -> io::Result<DnsPacket> {
        let mut buffer = [0u8; 65_535];
        let (bytes_read, _) = socket.recv_from(&mut buffer)?;
        let (_, response) = deserialize_packet(&buffer[..bytes_read]).unwrap();
        Ok(response)
    }
}
