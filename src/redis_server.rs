use crate::command_dispatcher::CommandDispatcher;
use crate::types::lib::Parser;
use std::io::{BufReader, BufWriter, Error, Write};
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
        let reader = BufReader::new(&stream);
        let mut writer = BufWriter::new(&stream);

        let mut parser = Parser::new(reader);
        loop {
            let parsed = parser.next();
            match parsed {
                Ok(value) => {
                    log::info!("Parsed: {:?}", value);
                    let command_prefixes = match value {
                        crate::types::lib::RequestPrimitive::Array(a) => {
                            if a.elements.len() == 1 || a.elements.len() == 2 {
                                a.elements
                            } else {
                                a.elements[0..2].to_vec()
                            }
                        }
                        _ => vec![value],
                    };
                    let command_result = self.dispatcher.dispatch(command_prefixes);
                    match command_result {
                        Ok(command) => {
                            let result = command.execute(" ".to_string());
                            match result {
                                Ok(response) => {
                                    log::debug!("Sending: {}", response);
                                    writer.write_all(response.as_bytes()).unwrap();
                                    writer.flush().unwrap();
                                }
                                Err(e) => {
                                    log::error!("Failed to execute command: {:?}", e);
                                    let error_message = format!("Error: {}\r\n", e);
                                    writer.write_all(error_message.as_bytes()).unwrap();
                                    writer.flush().unwrap();
                                }
                            }
                        }
                        Err(e) => {
                            log::error!("Failed to find command: {:?}", e);
                            let error_message = format!("Error: {}\r\n", e);
                            writer.write_all(error_message.as_bytes()).unwrap();
                            writer.flush().unwrap();
                        }
                    }
                }
                Err(e) => {
                    log::error!("Failed to parse: {:?}", e);
                    writer.write_all(b"-ERR ").unwrap();
                    writer.write_all(e.as_bytes()).unwrap();
                    writer.flush().unwrap();
                }
            }
        }
    }
}
