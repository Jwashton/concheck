use std::collections::HashMap;
use std::net::IpAddr;

use crate::result::TestResult;

pub fn format_header(ports: &Vec<u16>, longest_name: usize) -> String {
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

pub fn format_server(
    address: IpAddr,
    name: String,
    longest_name: usize,
    all_ports: &Vec<u16>,
    results: HashMap<u16, TestResult>,
) -> String {
    format!(
        "\t{: <15}\t{:width$}\t{}",
        address,
        name,
        format_results(all_ports, results),
        width = longest_name
    )
}

pub fn format_results(all_ports: &Vec<u16>, results: HashMap<u16, TestResult>) -> String {
    all_ports
        .iter()
        .map(|port| match results.get(port) {
            // Some(true) => "✅",
            Some(TestResult::Success) => "yes",
            // Some(false) => "❌",
            Some(TestResult::Failure(_, _)) => "no",
            Some(TestResult::Skipped) => " ",
            None => " ",
        })
        .collect::<Vec<&str>>()
        .join("\t")
}
