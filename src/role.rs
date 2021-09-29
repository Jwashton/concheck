use std::collections::HashSet;
use std::net::{IpAddr, ToSocketAddrs};

use serde::Deserialize;

use crate::server::ServerType;
use crate::services::Services;

#[derive(Deserialize, Debug)]
pub struct Role {
    name: String,
    services: Services,
    servers: Vec<String>,
}

type MaybeAddress = Result<IpAddr, std::io::Error>;

impl Role {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn services(&self) -> &Services {
        &self.services
    }

    pub fn ports(&self) -> HashSet<u16> {
        self.services.to_port_checks().keys().cloned().collect()
    }

    pub fn server_names(&self) -> &Vec<String> {
        &self.servers
    }

    pub fn addresses(&self) -> Vec<(String, MaybeAddress)> {
        self.servers
            .iter()
            .map(|name| (name.clone(), to_ip_addr(name.as_str())))
            .collect()
    }

    pub fn servers(&self) -> Vec<ServerType> {
        let port_checks = self.services().to_port_checks();

        self.addresses()
            .iter()
            .map(|(name, maybe_address)| match maybe_address {
                Ok(address) => {
                    ServerType::known(address.clone(), name.clone(), port_checks.clone())
                }
                Err(_) => ServerType::unknown(name.clone()),
            })
            .collect()
    }

    pub fn check_servers(&self, all_ports: Vec<u16>) -> Vec<ServerType> {
        let mut servers = self.servers();

        for server in &mut servers {
            server.check_ports(all_ports.clone());
        }

        for server in &mut servers {
            server.collect_results(all_ports.clone());
        }

        servers
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
