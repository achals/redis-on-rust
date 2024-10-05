use super::super::storage::Storage;
use super::Command;
use std::error::Error;
use std::sync::Arc;

const RESPONSE: &str = "%3\r\n+server\r\n+redis-on-rust\r\n+version\r\n+0.0.1\r\n+proto\r\n:3\r\n";

pub struct HelloCommand {}

impl HelloCommand {
    pub fn new() -> Arc<HelloCommand> {
        Arc::new(HelloCommand {})
    }
}

impl Command for HelloCommand {
    fn execute(&self, _: String) -> Result<&str, Box<dyn Error>> {
        Ok(RESPONSE)
    }
}
