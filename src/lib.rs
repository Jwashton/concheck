use std::collections::HashSet;

pub mod reporting;
pub mod role;
pub mod services;

pub mod net_check {
    use std::collections::HashMap;
    use std::net::{IpAddr, SocketAddr, TcpStream};
    use std::time::Duration;

    pub fn test_port(address: &SocketAddr, enabled: &bool) -> bool {
        match TcpStream::connect_timeout(address, Duration::from_secs(1)) {
            Ok(_) => *enabled,
            Err(_) => !enabled,
        }
    }

    pub fn check_server(address: IpAddr, port_checks: &HashMap<u16, bool>) -> HashMap<u16, bool> {
        port_checks
            .iter()
            .map(|(number, enabled)| {
                let socket = SocketAddr::new(address, *number);
                (*number, test_port(&socket, enabled))
            })
            .collect()
    }
}

pub fn collect_ports(roles: &Vec<role::Role>) -> Vec<u16> {
    let mut all_ports = roles
        .iter()
        .fold(HashSet::new(), |ports, role| &ports | &role.ports())
        .into_iter()
        .collect::<Vec<u16>>();

    all_ports.sort();

    all_ports
}