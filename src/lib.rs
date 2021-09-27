pub mod role;
pub mod services;

pub mod net_check {
    use std::net::{SocketAddr, TcpStream};
    use std::time::Duration;

    pub fn test_port(address: &SocketAddr, enabled: &bool) -> bool {
        match TcpStream::connect_timeout(address, Duration::from_secs(1)) {
            Ok(_) => *enabled,
            Err(_) => !enabled,
        }
    }
}
