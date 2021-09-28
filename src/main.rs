use std::collections::HashSet;
use std::error::Error;
use std::fs::File;

use serde_yaml;

use concheck::net_check;
use concheck::reporting;
use concheck::role::Role;

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
        reporting::format_header(&all_ports, longest_name.unwrap().chars().count())
    );

    for role in &roles {
        let port_checks = role.services().to_port_checks();

        println!("{}:", role.name());

        for (name, maybe_address) in role.addresses() {
            match maybe_address {
                Ok(address) => {
                    let results = net_check::check_server(address, &port_checks);
                    // println!("\t{}\t{} => {:?}", address, name, results)
                    println!(
                        "{}",
                        reporting::format_server(
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
