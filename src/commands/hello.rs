use super::Command;
use crate::types::lib;
use crate::types::lib::RESPType;
use std::error::Error;
use std::sync::Arc;

pub struct HelloCommand {}

impl HelloCommand {
    pub fn new() -> Arc<HelloCommand> {
        Arc::new(HelloCommand {})
    }
}

impl Command for HelloCommand {
    fn execute(&self, _: RESPType) -> Result<RESPType, Box<dyn Error>> {
        Ok(RESPType::Map(lib::Map {
            elements: vec![
                (
                    RESPType::BulkString("server".to_string()),
                    RESPType::BulkString("redis-on-rust".to_string()),
                ),
                (
                    RESPType::BulkString("version".to_string()),
                    RESPType::BulkString("0.0.1".to_string()),
                ),
                (
                    RESPType::BulkString("proto".to_string()),
                    RESPType::Integer(3),
                ),
            ],
        }))
    }
}
