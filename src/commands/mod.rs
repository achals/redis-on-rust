pub(crate) mod command_meta;
pub(crate) mod hello;

use crate::types::lib::RESPType;
use std::error::Error;
use std::fmt;

pub trait Command {
    fn execute(&self, command: RESPType) -> Result<RESPType, Box<dyn Error>>;
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
