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

fn format_header(ports: &Vec<u16>, longest_name: usize) -> String {
    let header = ports
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

fn format_server(
    address: IpAddr,
    name: String,
    longest_name: usize,
    all_ports: &Vec<u16>,
    results: HashMap<u16, bool>,
) -> String {
    format!(
        "\t{: <15}\t{:width$}\t{}",
        address,
        name,
        format_results(all_ports, results),
        width = longest_name
    )
}

fn format_results(all_ports: &Vec<u16>, results: HashMap<u16, bool>) -> String {
    all_ports
        .iter()
        .map(|port| match results.get(port) {
            Some(true) => "yes",
            Some(false) => "no",
            None => " ",
        })
        .collect::<Vec<&str>>()
        .join("\t")
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./inventory.yml")?;
    let roles: Vec<Role> = serde_yaml::from_reader(file)?;

    let mut all_ports = roles
        .iter()
        .fold(HashSet::new(), |ports, role| &ports | &role.ports())
        .into_iter()
        .collect::<Vec<u16>>();

    all_ports.sort();

    let all_names = roles
        .iter()
        .map(|role| role.servers())
        .flatten()
        .collect::<Vec<&String>>();

    let longest_name = all_names
        .iter()
        .max_by(|x, y| x.chars().count().cmp(&y.chars().count()));

    println!(
        "{}",
        format_header(&all_ports, longest_name.unwrap().chars().count())
    );

    for role in &roles {
        let port_checks = role.services().to_port_checks();

        println!("{}:", role.name());

        for (name, maybe_address) in role.addresses() {
            match maybe_address {
                Ok(address) => {
                    let results = check_server(address, &port_checks);
                    // println!("\t{}\t{} => {:?}", address, name, results)
                    println!(
                        "{}",
                        format_server(
                            address,
                            name,
                            longest_name.unwrap().chars().count(),
                            &all_ports,
                            results
                        )
                    )
                }
                Err(error) => println!("{} -> {}", name, error),
            }
        }
    }

    Ok(())
}
