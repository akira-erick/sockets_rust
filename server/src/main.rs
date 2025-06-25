use std::io::{self, BufReader, BufRead, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(stream: TcpStream) -> io::Result<()> {
    let peer_addr = stream.peer_addr()?;
    println!("Client connected: {}", peer_addr);

    let mut reader = BufReader::new(stream.try_clone()?);
    let mut writer = stream;

    loop {
        let mut line = String::new();
        let bytes_read = reader.read_line(&mut line)?;

        if bytes_read == 0 {
            println!("Client disconnected: {}", peer_addr);
            break;
        }

        let trimmed_line = line.trim_end();
        println!("Received from {}: {}", peer_addr, trimmed_line);

        let response = format!("{}\n", trimmed_line);

        writer.write_all(response.as_bytes())?;
        writer.flush()?;

        println!("Echoed {} bytes back to {}", response.len(), peer_addr);
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let ip = "0.0.0.0:7878";
    let listener = TcpListener::bind(ip).expect("Could not bind to address");
    println!("Echo server running on {}", ip);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    if let Err(e) = handle_client(stream) {
                        eprintln!("Error handling client: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
    Ok(())
}
