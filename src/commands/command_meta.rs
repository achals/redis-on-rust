use crate::commands::Command;
use crate::types::lib::{Array, Map, RESPType};
use std::error::Error;
use std::sync::Arc;

pub struct CommandMeta {}

impl CommandMeta {
    pub fn new() -> Arc<CommandMeta> {
        Arc::new(CommandMeta {})
    }
}

impl Command for CommandMeta {
    fn execute(&self, _: RESPType) -> Result<RESPType, Box<dyn Error>> {
        Ok(RESPType::Map(Map {
            elements: vec![(
                RESPType::BulkString("HELLO".to_string()),
                RESPType::Map(Map {
                    elements: vec![
                        (
                            RESPType::BulkString("summary".to_string()),
                            RESPType::BulkString("Hello command".to_string()),
                        ),
                        (
                            RESPType::BulkString("since".to_string()),
                            RESPType::BulkString("0.0.1".to_string()),
                        ),
                        (
                            RESPType::BulkString("group".to_string()),
                            RESPType::BulkString("connection".to_string()),
                        ),
                        (
                            RESPType::BulkString("complexity".to_string()),
                            RESPType::BulkString("O(1)".to_string()),
                        ),
                        (
                            RESPType::BulkString("doc_flags".to_string()),
                            RESPType::Array(Array {
                                elements: vec![RESPType::BulkString("syscmd".to_string())],
                            }),
                        ),
                        (
                            RESPType::BulkString("arguments".to_string()),
                            RESPType::Array(Array { elements: vec![] }),
                        ),
                    ],
                }),
            )],
        }))
    }
}
