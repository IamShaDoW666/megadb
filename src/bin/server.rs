use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Create a TCP listener bound to the specified address
    let listener = TcpListener::bind("127.0.0.1:3738")
        .await
        .expect("Unable to bind to address");
    loop {
        let (socket, addr) = listener
            .accept()
            .await
            .expect("Failed to accept connection");
        tokio::spawn(async move {
            handle_connection(socket).await;
        });
    }
}

async fn handle_connection(stream: tokio::net::TcpStream) {
    println!("New connection: {:?}", stream.peer_addr());
}
