use std::io::{self, Write};
use std::net::TcpStream;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:3738")?;
    let mut message = String::new();
    loop {
        print!(">");
        io::stdout().flush()?;
        io::stdin()
            .read_line(&mut message)
            .expect("Enter a valid message!");

        stream.write(message.as_bytes());
    }
}
