use std::fs::File;
use std::io;
use std::net::UdpSocket;
use std::os::fd::FromRawFd;

#[cfg(target_os = "macos")]
pub fn open_tunnel() -> io::Result<(File, String)> {
    let mut tunnel_id: u8 = 0;
    let fd = unsafe { open_utun(&mut tunnel_id) };
    if fd < 0 {
        return Err(io::Error::last_os_error())
    }

    let tunnel_name = format!("utun{}", tunnel_id);
    let socket = unsafe { File::from_raw_fd(fd) };

    Ok((socket, tunnel_name))
}

#[cfg(target_os = "macos")]
extern "C" {
    fn open_utun(tun_id: &mut u8) -> i32;
}

#[cfg(test)]
mod tests {
    use std::{io, thread};
    use std::io::{Read, Write};

    use nix::sys::socket::{socket, AddressFamily, SockType, SockFlag};
    use nix::sys::socket::{connect, SockaddrIn};
    use std::os::unix::io::RawFd;
    use std::ffi::CString;
    use std::mem;
    use std::os::fd::OwnedFd;

    use crate::open_tunnel;

    #[cfg(target_os = "macos")]
    #[test]
    fn test() -> io::Result<()> {
        let (mut file, name) = open_tunnel()?;
        println!("Connected to tunnel: {}", name);

        let default_device_name = pcap::Device::lookup().unwrap().unwrap().name;
        println!("Forwarding requests to {}", &default_device_name);

        let mut cap = pcap::Capture::from_device("en0")
            .unwrap()
            .timeout(100)
            .promisc(true)
            .open()
            .unwrap();

        let datalink_type = cap.get_datalink().0;
        println!("Data link type: {}", datalink_type);

        let mut buffer = [0u8; 65_535];
        loop {
            match file.read(&mut buffer) {
                Ok(bytes_received) => {
                    // println!("Received {} bytes", bytes_received);
                    println!("Request: {:02x?}", &buffer[..bytes_received]);

                    match cap.sendpacket(&buffer[..bytes_received]) {
                        Ok(_) => {
                            // println!("Successfully sent packet to {}", &default_device_name);
                        },
                        Err(e) => {
                            eprintln!("Dropped packet sending to {}: {}", &default_device_name, e);
                            continue;
                        },
                    }

                    match cap.next_packet() {
                        Ok(packet) => {
                            println!("Response: {:02x?}", &packet.data);
                            file.write_all(&packet.data)?;
                        },
                        Err(e) => {
                            eprintln!("Dropped packet receiving from {}: {}", &default_device_name, e);
                        },
                    }

                    // file.write(&buffer[..bytes_received])?;
                    // file.write("wassup".as_bytes())?;
                    // println!("{:#04x?}", &buffer[0..49]);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }

    #[test]
    fn open_device() -> io::Result<()> {
        let default_device_name = pcap::Device::lookup().unwrap().unwrap();

        println!("Starting...");

        let mut cap = pcap::Capture::from_device("utun7")
            .unwrap()
            .timeout(100)
            .promisc(true)
            .open()
            .unwrap();

        while let Ok(packet) = cap.next_packet() {
            println!("Packet length: {}", packet.len());
            // println!("{:?}", packet.data);
        }

        Ok(())
    }
}
