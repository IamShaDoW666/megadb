use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:3738")
        .await
        .expect("Unable to bind to address");
    loop {
        let (socket, _) = listener
            .accept()
            .await
            .expect("Failed to accept connection");
        tokio::spawn(async move {
            handle_connection(socket).await;
        });
    }
}

async fn handle_connection(mut stream: tokio::net::TcpStream) {
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
            println!("{}>{}", stream.peer_addr().unwrap(), msg);
            stream
                .write_all(b"Message received\n")
                .await
                .expect("Failed to send response");
        }
    }
}
