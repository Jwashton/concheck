use std::error::Error;

use concheck::inventory::Inventory;
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
        println!("{}:", role.name());
        let rx = role.check_servers();

        for _ in 0..(role.servers().len()) {
            let (address, name, results) = rx.recv().unwrap();

            println!(
                "{}",
                reporting::format_server(address, name, longest_name_length, &all_ports, results)
            )
        }
    }

    Ok(())
}
