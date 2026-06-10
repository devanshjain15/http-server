use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    // buffer
    let mut buffer = [0; 1024];
    // copying raw bytes
    stream.read(&mut buffer)?;
    let request = String::from_utf8_lossy(&buffer); 

    // parsing 
    let (method, path, version) = parse_request_line(&request); 
    println!("{method}, {path}, {version}"); 

    // sending http response 
    let status = "HTTP/1.1 200 OK"; 
    let contents = "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<title>Hello World</title>\n</head>\n<body>\n<h1>Hello from the server!</h1>\n</body>\n</html>";
    let content_len = contents.len(); 
    let response = format!(
        "{status}\r\nContent-Type: text/html; charset=UTF-8\r\nContent-Length: {content_len}\r\n\r\n{contents}"
    );
    stream.write_all(response.as_bytes())?; 
    Ok(())
}

fn parse_request_line(request: &str) -> (&str, &str, &str) { 
    // method, path, version
    let line= request.lines().next().unwrap(); 
    let first_line: Vec<&str> = line.split_whitespace().collect(); 
    (first_line[0], first_line[1], first_line[2]) 
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
