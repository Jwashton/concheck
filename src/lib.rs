use std::collections::HashSet;

pub mod inventory;
pub mod reporting;
pub mod result;
pub mod role;
pub mod server;
pub mod services;

pub mod net_check {
    use std::collections::HashMap;
    use std::net::{IpAddr, SocketAddr, TcpStream};
    use std::time::Duration;

    use crate::result::TestResultKind;

    pub fn test_port(address: &SocketAddr, enabled: &bool) -> TestResultKind {
        let connection = TcpStream::connect_timeout(address, Duration::from_secs(1));

        TestResultKind::from_result(connection, *enabled)
    }

    pub fn check_server(address: IpAddr, port_checks: &HashMap<u16, bool>) -> HashMap<u16, TestResultKind> {
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