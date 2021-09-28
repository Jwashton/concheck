use std::error::Error;
use std::fs::File;

use serde_yaml;

use concheck::net_check;
use concheck::reporting;
use concheck::role::Role;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./inventory.yml")?;
    let roles: Vec<Role> = serde_yaml::from_reader(file)?;

    let all_ports = concheck::collect_ports(&roles);

    let all_server_names = roles
        .iter()
        .map(|role| role.servers())
        .flatten()
        .collect::<Vec<&String>>();

    let longest_name_length = all_server_names
        .iter()
        .map(|name| name.chars().count())
        .max()
        .unwrap();

    println!(
        "{}",
        reporting::format_header(&all_ports, longest_name_length)
    );

    for role in &roles {
        let port_checks = role.services().to_port_checks();

        println!("{}:", role.name());

        for (name, maybe_address) in role.addresses() {
            match maybe_address {
                Ok(address) => {
                    let results = net_check::check_server(address, &port_checks);
                    println!(
                        "{}",
                        reporting::format_server(
                            address,
                            name,
                            longest_name_length,
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
