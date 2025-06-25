use core::str;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    let ip_to_connect =
        std::env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:7878".to_string());
    let mut stream = TcpStream::connect(&ip_to_connect)?;
    println!("Connected to server at {}", ip_to_connect);
    println!("Type messages to send to the server. Type 'exit' to quit.");

    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin.lock());

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        reader.read_line(&mut input)?;

        let input = input.trim();

        if input == "exit" {
            println!("Exiting client.");
            break;
        }

        if input.is_empty() {
            continue;
        }

        stream.write_all(input.as_bytes())?;
        stream.write_all(b"\n")?;
        stream.flush()?;

        let mut buffer = vec![0; 512];
        let bytes_read = stream.read(&mut buffer)?;

        if bytes_read == 0 {
            println!("Server closed the connection.");
            break;
        } else {
            let response = str::from_utf8(&buffer[0..bytes_read])
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            print!("Server response: {}", response);
        }
    }
    println!("Client finished.");
    Ok(())
}
