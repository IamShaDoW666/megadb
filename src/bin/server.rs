use std::sync::Arc;

use megadb::core;
use megadb::parser;
use megadb::parser::Command;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let db = Arc::new(core::Store::new());
    let listener = TcpListener::bind("127.0.0.1:3738")
        .await
        .expect("Unable to bind to address");
    loop {
        let (socket, _) = listener
            .accept()
            .await
            .expect("Failed to accept connection");
        let db = db.clone();
        tokio::spawn(async move {
            handle_connection(socket, db).await;
        });
    }
}

async fn do_command(command: Command, db: Arc<core::Store>) -> Result<String, ()> {
    match command {
        Command::Get { key } => {
            let value = db.get(key.clone());
            match value {
                Some(v) => {
                    println!("Value for {}: {}", key, v);
                    Ok(format!("> Value for {}: {}", key, v))
                }
                None => {
                    println!("Key {} not found", key);
                    Err(())
                }
            }
        }
        Command::Set { key, value } => {
            let success = db.set(key, value);
            match success {
                Some(v) => {
                    println!("Key set successfully, previous value: {}", v);
                    Ok(format!("> Key set successfully, previous value: {}", v))
                }
                None => {
                    println!("Key set successfully, no previous value");
                    Ok("> Key set successfully, no previous value".to_string())
                }
            }
        }
        Command::Del { key } => {
            let success = db.delete(key);
            if success {
                println!("Key deleted successfully");
                Ok("> Key deleted successfully".to_string())
            } else {
                println!("Failed to delete key");
                Err(())
            }
        }
        Command::Unknown(s) => {
            println!("Unknown command: {}", s);
            Ok(format!("> Unknown command: {}", s))
        }
        Command::Invalid => {
            println!("Invalid command format");
            Ok("> Invalid command format".to_string())
        }
    }
}

async fn handle_connection(mut stream: tokio::net::TcpStream, db: Arc<core::Store>) {
    println!("New connection: {:?}", stream.peer_addr().unwrap());
    let mut buffer = [0_u8; 512];
    loop {
        let bytes_read = match stream.read(&mut buffer).await {
            Ok(0) => {
                println!("Client closed connection {}", stream.peer_addr().unwrap());
                break;
            }
            Ok(n) => n,
            Err(e) => {
                eprintln!("connection timed out {}", e);
                break;
            }
        };

        if bytes_read > 0 {
            let msg = String::from_utf8_lossy(&buffer[..bytes_read]);
            println!("NEW MESSAGE");
            println!("{}>{}", stream.peer_addr().unwrap(), msg);
            let command = parser::Command::from_string(msg.to_string());
            println!("Command parsed: {:?}", command);
            let res = do_command(command, db.clone()).await;
            match res {
                Ok(response) => {
                    println!("Response: {}", response);
                    stream
                        .write_all(format!("{}\n", response).as_bytes())
                        .await
                        .expect("Failed to send response")
                }
                Err(_) => {
                    println!("Error processing command");
                    stream
                        .write_all(b"Error processing command\n")
                        .await
                        .expect("Failed to send error response");
                }
            }
        }
    }
}
