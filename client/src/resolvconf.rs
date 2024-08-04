use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::ops::Add;

/// Get the address of the nameserver from resolv.conf, used to make DNS queries
pub(crate) fn get_resolvconf_addr() -> io::Result<String> {
    let file = File::open(RESOLVCONF_PATH)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("nameserver") {
            return Ok(String::from(&line[11..26]).add(":53"));
        }
    }

    panic!("No nameserver found in resolv.conf");
}

const RESOLVCONF_PATH: &str = "/etc/resolv.conf";

#[cfg(test)]
mod tests {
    use regex::Regex;
    use super::*;

    #[test]
    fn it_works() {
        let addr = get_resolvconf_addr().unwrap();
        let re = Regex::new(r"^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}:53$").unwrap();

        assert!(re.is_match(&addr), "Invalid nameserver address: {}", addr);
    }
}
