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

fn format_header(ports: HashSet<u16>, longest_name: usize) -> String {
    let mut ordered_ports = ports.iter().collect::<Vec<&u16>>();

    ordered_ports.sort();

    let header = ordered_ports
        .iter()
        .map(|port| port.to_string())
        .collect::<Vec<String>>()
        .join("\t");

    format!(
        "\t{: <15}\t{:width$}\t{}",
        "-",
        "-",
        header,
        width = longest_name
    )
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./inventory.yml")?;
    let roles: HashMap<String, Role> = serde_yaml::from_reader(file)?;

    let all_ports = roles.iter().fold(HashSet::new(), |ports, (_name, role)| {
        &ports | &role.ports()
    });
    let all_names = roles
        .iter()
        .map(|(_name, role)| role.servers())
        .flatten()
        .collect::<Vec<&String>>();

    let longest_name = all_names
        .iter()
        .max_by(|x, y| x.chars().count().cmp(&y.chars().count()));

    println!("{}", format_header(all_ports, longest_name.unwrap().chars().count()));

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
