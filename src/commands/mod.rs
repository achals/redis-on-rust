pub mod hello;

use super::storage::Storage;
use std::error::Error;
use std::fmt;

pub trait Command {
    fn execute(&self, command: String) -> Result<&str, Box<dyn Error>>;
}

pub struct CommandExecutionError {
    message: String,
}

impl fmt::Display for CommandExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Command Execution Error: {}", self.message)
    }
}

impl fmt::Debug for CommandExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Command Execution Error: {}", self.message)
    }
}
