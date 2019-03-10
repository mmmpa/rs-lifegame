use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct StandardError {
    details: String
}

impl StandardError {
    pub fn new(msg: &str) -> StandardError {
        StandardError { details: msg.to_string() }
    }
}

impl fmt::Display for StandardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for StandardError {
    fn description(&self) -> &str {
        &self.details
    }
}
