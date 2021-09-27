use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::net::{IpAddr, SocketAddr};

use serde_yaml;

use concheck::net_check;
use concheck::role::Role;

fn check_server(address: IpAddr, port_checks: &HashMap<u16, bool>) -> HashMap<u16, bool> {
    port_checks
        .iter()
        .map(|(number, enabled)| {
            let socket = SocketAddr::new(address, *number);
            (*number, net_check::test_port(&socket, enabled))
        })
        .collect()
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

        for (name, maybe_address) in role.addresses() {
            match maybe_address {
                Ok(address) => {
                    let results = check_server(address, &port_checks);
                    println!("\t{}\t{} => {:?}", address, name, results)
                }
                Err(error) => println!("{} -> {}", name, error),
            }
        }
    }

    Ok(())
}
