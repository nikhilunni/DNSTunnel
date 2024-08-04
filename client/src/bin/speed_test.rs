use std::io;
use std::io::Read;
use std::net::UdpSocket;
use std::time::SystemTime;

use serde::codec::{decode_dns_request_as_data, encode_data_as_dns_request};
use serde::deserializer::deserialize_packet;
use serde::DnsPacket;
use serde::serializer::serialize_dns_packet;


fn main() -> io::Result<()> {
    let server = "0.0.0.0:9090";
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    let mut bytes_sent: u64 = 0;
    let mut bytes_received: u64 = 0;
    let mut num_requests: u64 = 0;

    let (udp_tunnel, tunnel_name) = tunnel::open_tunnel()?;

    println!("Opened tunnel: {}", tunnel_name);

    let start = SystemTime::now();

    loop {
        unsafe {
            let (bytes_read, _) = udp_tunnel.recv_from(&mut NETWORK_BUFFER)?;
            let requests = encode_data_as_dns_request(&NETWORK_BUFFER[..bytes_read], "2cc.co");
            let mut responses: Vec<DnsPacket> = vec![];

            for request in requests {
                let serialized_request = serialize_dns_packet(&request)?;
                socket.send_to(&serialized_request, server)?;

                let response = get_response(&socket)?;
                responses.push(response);
            }
            bytes_sent += bytes_read as u64;

            let response = decode_dns_request_as_data(responses, "2cc.co");
            bytes_received += response.len() as u64;

            assert_eq!(Vec::from(&NETWORK_BUFFER[..bytes_read]), response);

            num_requests += 1;

            let seconds_elapsed = start.elapsed().unwrap().as_secs();
            if num_requests % 1000 == 0 && seconds_elapsed > 0 {
                let kbytes_sent_per_second = bytes_sent / (1 << 20) / seconds_elapsed;
                let kbytes_received_per_second = bytes_received / (1 << 20) / seconds_elapsed;
                println!("Sent: {}Mib / sec, Received: {}Mib / sec", kbytes_sent_per_second, kbytes_received_per_second);
            }
        }
    }
}

static mut NETWORK_BUFFER: [u8; 65_535] = [0u8; 65_535];

unsafe fn get_response(socket: &UdpSocket) -> io::Result<DnsPacket> {
    let (bytes_read, _) = socket.recv_from(&mut NETWORK_BUFFER)?;
    let (_, response) = deserialize_packet(&NETWORK_BUFFER[..bytes_read]).unwrap();
    Ok(response)
}
