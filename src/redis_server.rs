use std::io::{BufRead, BufReader, BufWriter, Error, Write};
use std::net::{TcpListener, TcpStream};

pub struct RedisServer {
    tcp_listener: TcpListener,
}

impl RedisServer {
    pub fn new(port: u16) -> RedisServer {
        RedisServer {
            tcp_listener: TcpListener::bind(format!("127.0.0.1:{}", port.to_string())).unwrap(),
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
        let mut reader = BufReader::new(&stream);
        let mut writer = BufWriter::new(&stream);

        loop {
            let mut buffer = String::new();
            match reader.read_line(&mut buffer) {
                Ok(0) => {
                    log::info!("Connection closed");
                    break;
                }
                Ok(_) => {
                    log::info!("Received: {}", buffer);
                    writer.write_all(buffer.as_bytes()).unwrap();
                }
                Err(e) => {
                    log::error!("Failed to read from stream: {:?}", e);
                    break;
                }
            }
        }
    }

}