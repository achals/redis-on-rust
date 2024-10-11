use crate::command_dispatcher::CommandDispatcher;
use crate::storage::memory::InMemoryStorage;
use crate::types::lib::RESPType;
use crate::types::lib::{Parser, Writer};
use std::io::{BufReader, BufWriter, Error};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

pub struct RedisServer {
    tcp_listener: TcpListener,
    dispatcher: Box<CommandDispatcher>,
}

impl RedisServer {
    pub fn new(port: u16) -> RedisServer {
        let storage = Arc::new(InMemoryStorage::new());
        RedisServer {
            tcp_listener: TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap(),
            dispatcher: CommandDispatcher::new(storage),
        }
    }

    pub fn start(&mut self) -> Result<(), Error> {
        for stream in self.tcp_listener.incoming() {
            match stream {
                Ok(stream) => {
                    log::info!("Connection established: {:?}", stream.peer_addr()?);

                    self.handle_connection(stream);
                }
                Err(e) => {
                    log::error!("Failed to establish connection: {:?}", e);
                }
            }
        }
        Ok(())
    }

    fn handle_connection(&self, stream: TcpStream) {
        let mut parser = Parser::new(BufReader::new(&stream));
        let mut writer = Writer::new(BufWriter::new(&stream));
        loop {
            let parsed = parser.next();
            let response = match parsed {
                Ok(value) => {
                    log::info!("Parsed: {:?}", value);
                    let command_prefixes = match value {
                        RESPType::Array(a) => {
                            if a.elements.len() == 1 || a.elements.len() == 2 {
                                a.elements
                            } else {
                                a.elements[0..2].to_vec()
                            }
                        }
                        _ => vec![value],
                    };
                    let command_result = self.dispatcher.dispatch(command_prefixes.clone());
                    match command_result {
                        Ok(command) => {
                            let result = command.execute(command_prefixes[0].clone());
                            match result {
                                Ok(response) => writer.write(response),
                                Err(e) => {
                                    log::error!("Failed to execute command: {:?}", e);

                                    let error_resp = RESPType::Error(e.to_string());
                                    writer.write(error_resp)
                                }
                            }
                        }
                        Err(e) => {
                            log::error!("Failed to find command: {:?}", e);

                            let error_resp = RESPType::Error(e.to_string());
                            writer.write(error_resp)
                        }
                    }
                }
                Err(e) => {
                    if e == "Empty Command" {
                        break;
                    }
                    log::debug!("Failed to parse: {:?}", e);
                    let error_resp = RESPType::Error(e.to_string());
                    writer.write(error_resp)
                }
            };
            match response.and(writer.flush()) {
                Ok(_) => (),
                Err(e) => {
                    log::error!("Failed to flush: {:?}", e);
                    break;
                }
            }
        }
    }
}
