// Graphr error definitions

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct NodeError {
    details: String
}

impl NodeError {
    pub fn new(msg: &str) -> NodeError {
        NodeError{details: msg.to_string()}
    }
}

impl fmt::Display for NodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for NodeError {
    fn description(&self) -> &str {
        &self.details
    }
}
