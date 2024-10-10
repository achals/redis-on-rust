use super::commands::Command;
use crate::commands::command_meta::CommandMeta;
use crate::commands::hello::HelloCommand;
use crate::types::lib::RESPType;
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
            command_prefixes: vec![
                ("HELLO".to_string(), HelloCommand::new()),
                ("COMMAND DOCS".to_string(), CommandMeta::new()),
            ],
        })
    }

    fn extract_commands(command: Vec<RESPType>) -> Vec<String> {
        let mut bulk_string_prefixes = Vec::new();
        for c in command {
            match c {
                RESPType::BulkString(s) => {
                    bulk_string_prefixes.push(s);
                }
                _ => break,
            }
        }

        if bulk_string_prefixes.len() == 1 {
            bulk_string_prefixes
        } else {
            vec![
                bulk_string_prefixes[0].clone(),
                format!("{} {}", bulk_string_prefixes[0], bulk_string_prefixes[1]),
            ]
        }
    }

    pub(crate) fn dispatch(
        &self,
        command: Vec<RESPType>,
    ) -> Result<Arc<dyn Command>, Box<dyn Error>> {
        let command_choices = Self::extract_commands(command);
        for command_choice in command_choices.clone() {
            for (prefix, command) in &self.command_prefixes {
                if command_choice.to_uppercase().eq(prefix) {
                    log::info!("Dispatching command: {}", command_choice);
                    return Ok(command.clone());
                }
            }
        }

        Err(Box::new(CommandError {
            message: format!("Unknown command: {}", command_choices.last().unwrap()),
        }))
    }
}
