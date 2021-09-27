use std::collections::HashSet;
use std::net::{IpAddr, ToSocketAddrs};

use serde::Deserialize;

use crate::services::Services;

#[derive(Deserialize, Debug)]
pub struct Role {
    services: Services,
    servers: Vec<String>,
}

type MaybeAddress = Result<IpAddr, std::io::Error>;

impl Role {
    pub fn services(&self) -> &Services {
        &self.services
    }

    pub fn ports(&self) -> HashSet<u16> {
        self.services.to_port_checks().keys().cloned().collect()
    }

    pub fn servers(&self) -> &Vec<String> {
        &self.servers
    }

    pub fn addresses(&self) -> Vec<(String, MaybeAddress)> {
        self.servers
            .iter()
            .map(|name| (name.clone(), to_ip_addr(name.as_str())))
            .collect()
    }
}

fn to_ip_addr(name: &str) -> MaybeAddress {
    let mut result = (name, 22).to_socket_addrs()?;

    match result.next() {
        Some(address) => Ok(address.ip()),
        None => Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("No addresses found for: {}", name),
        )),
    }
}
