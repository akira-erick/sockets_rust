use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let peer_addr = stream.peer_addr()?;
    println!("Client connected: {}", peer_addr);

    let mut buffer = [0; 1024];
    loop {
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {
            println!("Client disconnected: {}", peer_addr);
            break;
        }

        stream.write_all(&buffer[..bytes_read])?;
        println!("Echoed {} bytes back to {}", bytes_read, peer_addr);
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let ip = "127.0.0.1:7878";
    let listener = TcpListener::bind(ip).expect("Could not bind to address");
    println!("Echo server rinning on {}", ip);

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
