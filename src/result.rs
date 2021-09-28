pub enum TestResult {
    Success,
    Failure(bool, bool),
    Skipped
}

use TestResult::{Success, Failure};

impl TestResult {
    pub fn from_result<A, B>(result: Result<A, B>, expected: bool) -> TestResult {
        match (result, expected) {
            (Ok(_), true) => Success,
            (Ok(_), false) => Failure(expected, true),
            (Err(_), true) => Failure(expected, false),
            (Err(_), false) => Success
        }
    }
}