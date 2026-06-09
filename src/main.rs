use std::io::Read;
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    // buffer
    let mut buffer = [0; 1024];
    // copying raw bytes
    stream.read(&mut buffer)?;
    println!("{}", String::from_utf8_lossy(&buffer));
    Ok(())
}

fn main() -> std::io::Result<()> {
    // open socket, bind and listen
    let listener = TcpListener::bind("127.0.0.1:8000")?;

    // accept established connections
    for stream in listener.incoming() {
        handle_client(stream?)?;
    }

    Ok(())
}
