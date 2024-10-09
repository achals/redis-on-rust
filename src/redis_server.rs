use crate::command_dispatcher::CommandDispatcher;
use crate::types::lib::RESPType;
use crate::types::lib::{Parser, Writer};
use std::io::{BufReader, BufWriter, Error};
use std::net::{TcpListener, TcpStream};

pub struct RedisServer {
    tcp_listener: TcpListener,
    dispatcher: Box<CommandDispatcher>,
}

impl RedisServer {
    pub fn new(port: u16) -> RedisServer {
        RedisServer {
            tcp_listener: TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap(),
            dispatcher: CommandDispatcher::new(),
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
            match parsed {
                Ok(value) => {
                    log::info!("Parsed: {:?}", value);
                    let command_prefixes = match value {
                        crate::types::lib::RESPType::Array(a) => {
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
                                Ok(response) => {
                                    writer.write(response).unwrap();
                                }
                                Err(e) => {
                                    log::error!("Failed to execute command: {:?}", e);

                                    let error_resp = RESPType::Error(e.to_string());
                                    writer.write(error_resp).unwrap();
                                }
                            }
                        }
                        Err(e) => {
                            log::error!("Failed to find command: {:?}", e);

                            let error_resp = RESPType::Error(e.to_string());
                            writer.write(error_resp).unwrap();
                        }
                    }
                }
                Err(e) => {
                    log::error!("Failed to parse: {:?}", e);
                    let error_resp = RESPType::Error(e.to_string());
                    writer.write(error_resp).unwrap();
                }
            }
        }
    }
}
