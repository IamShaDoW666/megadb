use config::{Config, Environment, File};
use serde::Deserialize;
use std::io::{self, Read, Result};
use std::net::{TcpListener, TcpStream};
use std::thread;

#[derive(Debug, Deserialize)]
struct ApplicationConfig {
    debug: bool,
    port: u16,
    server_addr: String,
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0_u8; 512];
    loop {
        let bytes_read = match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client {} closed connection", stream.peer_addr().unwrap());
                break;
            }
            Ok(n) => n,
            Err(e) => {
                eprintln!("Errored: {:?}", e);
                break;
            }
        };

        if bytes_read > 0 {
            let message = String::from_utf8_lossy(&buffer);
            println!(">>{}", message);
        }
    }
}

fn main() -> io::Result<()> {
    let s = Config::builder()
        .add_source(File::with_name("Config.toml"))
        .build()
        .unwrap();
    let settings: ApplicationConfig = s.try_deserialize();
    const ADDR: &str = "gww";
    let tcp_listener = TcpListener::bind(ADDR)?;
    println!("Listening on: {}", ADDR);
    for stream in tcp_listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Client connected: {}", stream.peer_addr().unwrap());
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                eprintln!("Error accepting connection!");
            }
        }
    }
    Ok(())
}
