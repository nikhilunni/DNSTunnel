use std::io;

#[cfg(target_os = "macos")]
pub fn open_tunnel() -> io::Result<(i32, String)> {
    let mut tunnel_id: u8 = 0;
    let fd = unsafe { open_utun(&mut tunnel_id) };
    if fd < 0 {
        return Err(io::Error::last_os_error())
    }

    let tunnel_name = format!("utun{}", tunnel_id);

    Ok((fd, tunnel_name))
}

#[cfg(target_os = "macos")]
extern "C" {
    fn open_utun(tun_id: &mut u8) -> i32;
}

#[cfg(test)]
mod tests {
    use std::thread;

    use crate::open_tunnel;

    #[cfg(target_os = "macos")]
    #[test]
    fn test() {
        let (fd, name) = open_tunnel().expect("Failed to open tunnel");
        println!("Connected to tunnel: {}", name);

        loop {
            thread::sleep(std::time::Duration::from_secs(10));
        }
    }
}
