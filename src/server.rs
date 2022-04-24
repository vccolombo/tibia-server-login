use std::{net::Ipv4Addr, str::FromStr};

pub struct Server {
    pub name: String,
    pub ip: Ipv4Addr,
    pub port: u16,
}

impl Server {
    pub fn default() -> Self {
        Self {
            name: String::from("localhost"),
            ip: Ipv4Addr::from_str("127.0.0.1").unwrap(),
            port: 7171,
        }
    }
}
