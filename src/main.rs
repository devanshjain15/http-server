use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    // buffer
    let mut buffer = [0; 1024];
    // copying raw bytes
    stream.read(&mut buffer)?;
    // println!("{}", String::from_utf8_lossy(&buffer));

    let status = "HTTP/1.1 200 OK"; 
    let contents = "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<title>Hello World</title>\n</head>\n<body>\n<h1>Hello from the server!</h1>\n</body>\n</html>";
    let content_len = contents.len(); 
    let response = format!(
        "{status}\r\nContent-Type: text/html; charset=UTF-8\r\nContent-Length: {content_len}\r\n\r\n{contents}"
    );
    stream.write_all(response.as_bytes())?; 
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
