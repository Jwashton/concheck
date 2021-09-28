use std::error::Error;

use concheck::inventory::Inventory;
use concheck::net_check;
use concheck::reporting;

fn main() -> Result<(), Box<dyn Error>> {
    let inventory = Inventory::from_file("./inventory.yml")?;
    let all_ports = inventory.all_ports();
    let longest_name_length = inventory.length_of_longest_server_name();

    println!(
        "{}",
        reporting::format_header(&all_ports, longest_name_length)
    );

    for role in &inventory.roles {
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
