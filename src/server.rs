use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::sync::mpsc;
use std::thread;

use crate::net_check;
use crate::result::{FailureKind, TestResultKind};

pub enum ServerType {
    Unknown(UnknownServer),
    Known(Server),
}

impl ServerType {
    pub fn unknown(name: String) -> ServerType {
        ServerType::Unknown(UnknownServer::new(name))
    }

    pub fn known(address: IpAddr, name: String, tests: HashMap<u16, bool>) -> ServerType {
        ServerType::Known(Server::new(address, name, tests))
    }

    pub fn check_ports(&mut self, ports: Vec<u16>) {
        match self {
            ServerType::Unknown(_server) => (),
            ServerType::Known(server) => server.check_ports(ports),
        }
    }

    pub fn collect_results(&mut self, ports: Vec<u16>) {
        match self {
            ServerType::Unknown(_server) => (),
            ServerType::Known(server) => server.collect_results(ports),
        }
    }

    pub fn failures(&self) -> Vec<FailureKind> {
        match self {
            ServerType::Unknown(server) => server.failures(),
            ServerType::Known(server) => server.failures(),
        }
    }
}

pub struct UnknownServer {
    name: String,
}

impl UnknownServer {
    pub fn new(name: String) -> UnknownServer {
        UnknownServer { name }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn failures(&self) -> Vec<FailureKind> {
        vec![FailureKind::NoAddress(self.name.clone())]
    }
}

pub struct Server {
    pub address: IpAddr,
    pub name: String,
    tests: HashMap<u16, bool>,
    pub results: HashMap<u16, TestResultKind>,
    receiver: Option<mpsc::Receiver<TestResultMsg>>,
}

type TestResultMsg = (u16, TestResultKind);

impl Server {
    pub fn new(address: IpAddr, name: String, tests: HashMap<u16, bool>) -> Server {
        Server {
            address,
            name,
            tests,
            results: HashMap::new(),
            receiver: None,
        }
    }

    pub fn test(
        &self,
        tx: mpsc::Sender<TestResultMsg>,
        port: u16,
    ) -> thread::JoinHandle<Result<(), mpsc::SendError<TestResultMsg>>> {
        let test = self.tests.get(&port).map(|expected| expected.clone());
        let address = self.address.clone();

        thread::spawn(move || {
            let result = match test {
                Some(expected) => {
                    let socket = SocketAddr::new(address, port);
                    net_check::test_port(&socket, &expected)
                }
                None => TestResultKind::Skipped,
            };

            tx.send((port, result))
        })
    }

    pub fn check_ports(&mut self, ports: Vec<u16>) {
        let mut children = vec![];
        let (tx, rx) = mpsc::channel();

        self.receiver = Some(rx);

        for port in &ports {
            children.push(self.test(tx.clone(), *port));
        }
    }

    pub fn collect_results(&mut self, ports: Vec<u16>) {
        match &self.receiver {
            Some(rx) => {
                for _ in ports {
                    let (port, result) = rx.recv().unwrap();
                    self.results.insert(port, result);
                }
            }
            None => panic!("Ports not tested yet!"),
        }
    }

    pub fn failures(&self) -> Vec<FailureKind> {
        self.results
            .iter()
            .filter_map(|(port, result)| match result {
                TestResultKind::Failure(expected, actual) => Some(FailureKind::bad_port(
                    self.address.clone(),
                    self.name.clone(),
                    *port,
                    *expected,
                    *actual,
                )),
                _ => None,
            })
            .collect()
    }
}
