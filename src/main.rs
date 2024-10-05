use std::thread;
use clap::Parser;

mod redis_server;
mod parser;
mod commands;
mod storage;

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    #[clap(short, long, default_value_t = 9000)]
    port: u16,
}


fn main() {
    let cli_args = Cli::parse();
    env_logger::init();

    log::info!("Starting Redis server on port {}", cli_args.port);
    let mut server = redis_server::RedisServer::new(cli_args.port);

    let t = thread::spawn(move || {
        server.start().expect("Failed to start server");
    });

    t.join().unwrap();


}
