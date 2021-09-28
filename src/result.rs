pub enum TestResultKind {
    Success,
    Failure(bool, bool),
    Skipped
}

use TestResultKind::{Success, Failure};

impl TestResultKind {
    pub fn from_result<A, B>(result: Result<A, B>, expected: bool) -> TestResultKind {
        match (result, expected) {
            (Ok(_), true) => Success,
            (Ok(_), false) => Failure(expected, true),
            (Err(_), true) => Failure(expected, false),
            (Err(_), false) => Success
        }
    }
}