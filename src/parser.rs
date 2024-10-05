use std::error::Error;
use std::fmt;
use crate::commands::hello::HelloCommand;
use super::commands::Command;

struct CommandParser {
    command_prefixes: Vec<(String, Box<dyn Command>)>,
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

impl CommandParser {
    fn new() -> CommandParser {
        CommandParser {
            command_prefixes: vec![("HELLO".to_string(), HelloCommand::new())],
        }
    }

    fn parse(self, command: &str) -> Result<Box<dyn Command>, Box<dyn Error>> {
        let mut parts = command.split_whitespace();
        let command_name = parts.next().ok_or("Empty Command")?;

        for (prefix, command) in self.command_prefixes {
            if command_name.to_uppercase().eq(&prefix) {
                return Ok(command);
            }
        }

        Err(Box::new(CommandError { message: format!("Unknown command: {}", command_name) }))
    }
}