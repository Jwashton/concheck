use std::env;
use std::error::Error;

use concheck::inventory::Inventory;
use concheck::reporting;
use concheck::server::ServerType;
use concheck::result::FailureKind;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let inventory = Inventory::from_file(filename)?;
    let all_ports = inventory.all_ports();
    let longest_name_length = inventory.length_of_longest_server_name();

    println!(
        "{}",
        reporting::format_header(&all_ports, longest_name_length)
    );

    let mut all_failures = vec![];

    for role in &inventory.roles {
        println!("{}:", role.name());
        let servers = role.check_servers(all_ports.clone());

        for server in servers {
            match &server {
                ServerType::Unknown(server) => {
                    println!("Could not resolve {}", server.name())
                },
                ServerType::Known(server) => {
                    println!(
                        "{}",
                        reporting::format_server(server.address, server.name.clone(), longest_name_length, &all_ports, server.results.clone())
                    )
                },
            };

            for failure in server.failures() {
                all_failures.push(failure);
            }
        }
    }

    println!("\n{} Failures found:\n", all_failures.len());

    for failure in all_failures {
        match failure {
            FailureKind::NoAddress(name) => {
                println!("Failure: could not resolve {}\n", name)
            }

            FailureKind::BadPort(result) => {
                println!("{}", result)
            }
        }
    }

    Ok(())
}
