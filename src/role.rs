use std::collections::{HashMap, HashSet};
use std::net::{IpAddr, ToSocketAddrs};
use std::sync::mpsc;
use std::thread;

use serde::Deserialize;

use crate::net_check;
use crate::result::TestResultKind;
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

    pub fn servers(&self) -> &Vec<String> {
        &self.servers
    }

    pub fn addresses(&self) -> Vec<(String, MaybeAddress)> {
        self.servers
            .iter()
            .map(|name| (name.clone(), to_ip_addr(name.as_str())))
            .collect()
    }

    pub fn check_servers(&self) -> mpsc::Receiver<(IpAddr, String, HashMap<u16, TestResultKind>)> {
        let port_checks = self.services().to_port_checks();
        let (tx, rx) = mpsc::channel();

        for (name, maybe_address) in self.addresses() {
            match maybe_address {
                Ok(address) => {
                    let my_tx = tx.clone();
                    let my_checks = port_checks.clone();

                    thread::spawn(move || {
                        let results = net_check::check_server(address, &my_checks);
                        my_tx.send((address, name, results)).unwrap();
                    });
                }
                Err(error) => println!("{} -> {}", name, error),
            }
        }

        rx
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
