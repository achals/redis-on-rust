use super::super::storage::Storage;
use super::Command;
use std::error::Error;

const RESPONSE: &str = "%3\r\n+server\r\n+redis-on-rust\r\n+version\r\n+0.0.1\r\n+proto\r\n:3\r\n";

pub struct HelloCommand {

}

impl HelloCommand {
    pub fn new() -> Box<HelloCommand> {
        Box::new(HelloCommand {})
    }
}

impl Command for HelloCommand {
    fn execute(&self, _: String, _: &mut dyn Storage) -> Result<&str, Box<dyn Error>> {
        Ok(RESPONSE)
    }
}