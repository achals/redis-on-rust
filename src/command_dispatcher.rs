use super::commands::Command;
use crate::commands::hello::HelloCommand;
use crate::types::lib::RequestPrimitive;
use std::error::Error;
use std::fmt;
use std::sync::Arc;

pub struct CommandDispatcher {
    command_prefixes: Vec<(String, Arc<dyn Command + Send + Sync>)>,
}

struct CommandError {
    message: String,
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Command Error: {}", self.message)
    }
}

impl fmt::Debug for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Command Error: {}", self.message)
    }
}

impl Error for CommandError {}

impl CommandDispatcher {
    pub fn new() -> Box<CommandDispatcher> {
        Box::new(CommandDispatcher {
            command_prefixes: vec![("HELLO".to_string(), HelloCommand::new())],
        })
    }

    pub(crate) fn dispatch(
        &self,
        command: RequestPrimitive,
    ) -> Result<Arc<dyn Command>, Box<dyn Error>> {
        let command_name = match command {
            RequestPrimitive::BulkString(s) => s,
            _ => {
                return Err(Box::new(CommandError {
                    message: "Invalid command".to_string(),
                }))
            }
        };
        for (prefix, command) in &self.command_prefixes {
            if command_name.to_uppercase().eq(prefix) {
                return Ok(command.clone());
            }
        }

        Err(Box::new(CommandError {
            message: format!("Unknown command: {}", command_name),
        }))
    }
}
