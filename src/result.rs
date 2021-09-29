use std::fmt;
use std::net::IpAddr;

#[derive(Clone)]
pub enum TestResultKind {
    Success,
    Failure(bool, bool),
    Skipped,
}

use TestResultKind::{Failure, Success};

impl TestResultKind {
    pub fn from_result<A, B>(result: Result<A, B>, expected: bool) -> TestResultKind {
        match (result, expected) {
            (Ok(_), true) => Success,
            (Ok(_), false) => Failure(expected, true),
            (Err(_), true) => Failure(expected, false),
            (Err(_), false) => Success,
        }
    }
}

pub enum FailureKind {
    NoAddress(String),
    BadPort(TestFailure),
}

impl FailureKind {
    pub fn bad_port(
        address: IpAddr,
        name: String,
        port: u16,
        expected: bool,
        actual: bool,
    ) -> FailureKind {
        FailureKind::BadPort(TestFailure::new(address, name, port, expected, actual))
    }
}

pub struct TestFailure {
    address: IpAddr,
    name: String,
    port: u16,
    expected: bool,
    actual: bool,
}

impl TestFailure {
    pub fn new(
        address: IpAddr,
        name: String,
        port: u16,
        expected: bool,
        actual: bool,
    ) -> TestFailure {
        TestFailure {
            address,
            name,
            port,
            expected,
            actual,
        }
    }
}

impl fmt::Display for TestFailure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Connection Failure for `{}` on server `{}` at `{}`:\n",
            self.port, self.name, self.address
        )?;
        write!(
            f,
            "\tExpected {} but connection {}\n",
            should_connect_output(self.expected),
            connection_result_output(self.actual)
        )
    }
}

fn should_connect_output(expected: bool) -> &'static str {
    match expected {
        true => "to connect",
        false => "to not connect",
    }
}

fn connection_result_output(result: bool) -> &'static str {
    match result {
        true => "succeeded",
        false => "failed",
    }
}
