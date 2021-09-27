use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::time::Duration;

use serde_yaml;

use concheck::Role;

fn test_port(address: &SocketAddr, enabled: &bool) -> bool {
    match TcpStream::connect_timeout(address, Duration::from_secs(1)) {
        Ok(_) => *enabled,
        Err(_) => !enabled,
    }
}

fn check_server(
    name: String,
    port_checks: &HashMap<u16, bool>,
) -> Result<HashMap<u16, bool>, String> {
    match format!("{}:22", name).to_socket_addrs() {
        Ok(mut addresses) => match addresses.next() {
            Some(mut address) => Ok(port_checks
                .iter()
                .map(|(number, enabled)| {
                    address.set_port(*number);
                    (*number, test_port(&address, enabled))
                })
                .collect()),
            None => Err(format!("No address found for {}", name)),
        },

        Err(error) => Err(format!("{}: {}", name, error)),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./inventory.yml")?;
    let roles: HashMap<String, Role> = serde_yaml::from_reader(file)?;

    let mut all_ports: HashSet<u16> = HashSet::new();

    for (_name, role) in &roles {
        all_ports = &all_ports | &role.ports();
    }

    println!("All ports: {:?}", all_ports);

    for (name, role) in &roles {
        let port_checks = role.services().to_port_checks();

        println!("{}:", name);

        for server in role.servers() {
            match check_server(server.to_string(), &port_checks) {
                Ok(results) => {
                    println!("\t{} => {:?}", server, results)
                }
                Err(msg) => println!("{}", msg),
            }
        }
    }

    Ok(())
}
