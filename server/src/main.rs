use std::io;
use std::net::UdpSocket;
use clap::Parser;
use serde::deserializer::deserialize_packet;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Port to bind to
    #[arg(short, long, value_name = "port", default_value = "9090")]
    port: u16,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let sock = UdpSocket::bind(format!("0.0.0.0:{}", args.port)).expect(
        format!("Could not bind to address {}", args.port).as_str()
    );

    let mut buffer = [0u8; 65_535];
    loop {
        let (len, addr) = sock.recv_from(&mut buffer)?;
        // let (_, packet) = deserialize_packet(&buffer[..len]).unwrap();
        // println!("Received packet: {:x?}", packet);

        let len = sock.send_to(&buffer[..len], addr)?;
        // println!("{:?} bytes sent", len);
    }
}
