use tokio::{
    io::{self, AsyncBufReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::mpsc,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Establish a connection to the server
    let stream = TcpStream::connect("127.0.0.1:3738").await?;
    println!("Successfully connected to server at 127.0.0.1:3738");

    let (tx, mut rx) = mpsc::channel::<String>(32);
    let (reader, mut writer) = stream.into_split();
    let mut reader = io::BufReader::new(reader);

    // 2. Spawn a task to read from the server and print to stdout
    tokio::spawn(async move {
        let mut line = String::new();
        loop {
            match reader.read_line(&mut line).await {
                Ok(0) => {
                    println!("\nServer closed the connection.");
                    break;
                }
                Ok(_) => {
                    // Erase the current line, print the server message, then reprint the prompt
                    print!("\r{}\rServer: {}", " ".repeat(100), line);
                    print!("> ");
                    io::stdout().flush().await.unwrap();
                    line.clear();
                }
                Err(e) => {
                    eprintln!("Error reading from server: {}", e);
                    break;
                }
            }
        }
    });

    // 3. Spawn a task to read user input from stdin
    let input_tx = tx.clone();
    tokio::spawn(async move {
        let mut stdin_buffer = String::new();
        let mut stdin_buffered = io::BufReader::new(io::stdin());
        loop {
            print!("> ");
            io::stdout().flush().await.unwrap();
            stdin_buffer.clear();
            if stdin_buffered.read_line(&mut stdin_buffer).await.is_err() {
                break;
            }
            if input_tx.send(stdin_buffer.clone()).await.is_err() {
                eprintln!("Failed to send command to the writer task.");
                break;
            }
            if stdin_buffer.trim().eq_ignore_ascii_case("exit") {
                break;
            }
        }
    });

    // 4. Main task to send commands to the server
    while let Some(line) = rx.recv().await {
        if line.trim().eq_ignore_ascii_case("exit") {
            println!("Exiting.");
            break;
        }
        if writer.write_all(line.as_bytes()).await.is_err() {
            eprintln!("Failed to write to server.");
            break;
        }
    }

    Ok(())
}
