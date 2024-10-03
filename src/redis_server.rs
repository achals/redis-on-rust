use std::io::{Error};
use std::net::{TcpListener};

pub struct RedisServer {
    tcp_listener: TcpListener,
}

impl RedisServer {
    pub fn new(port: u16) -> RedisServer {
        RedisServer {
            tcp_listener: TcpListener::bind( format!("127.0.0.1:{}", port.to_string())).unwrap(),
        }
    }

    pub fn start(&mut self) -> Result<(), Error> {
            for stream in self.tcp_listener.incoming() {
                match stream {
                    Ok(stream) => {
                        log::info!("Connection established: {:?}", stream.peer_addr()?);

                        // handle_connection(stream);
                    }
                    Err(e) => {
                        log::error!("Failed to establish connection: {:?}", e);
                    }
                }
            }
            Ok(())
    }
}