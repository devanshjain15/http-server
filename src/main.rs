use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Request {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl Request {
    pub fn new(stream: &TcpStream) -> Self {
        // copying and reading incoming raw bytes in chunks
        let mut reader = BufReader::new(stream);
        let mut buffer: Vec<String> = Vec::new();

        loop {
            let mut chunk = String::new();
            reader.read_line(&mut chunk).unwrap();
            if chunk == "\r\n" {
                break;
            }
            buffer.push(chunk.trim_end().to_string());
        }

        // parsing headers
        let mut headers = HashMap::new(); 

        // determining content-length
        let mut content_length = 0;
        for (i, chunk) in buffer.iter().enumerate() {
            if i == 0 { 
                continue; 
            }

            let header: Vec<&str> = chunk.splitn(2, ':').collect(); 
            if header[0] == "Content-Length" {
                content_length = header[1].trim().parse().unwrap();
            }
            headers.insert(header[0].to_string(), header[1].trim().to_string()); 
        }
        
        let mut body = String::new(); 
        if content_length != 0 {
            // finding the end of the request based on the Content-Length
            let mut body_buf = vec![0; content_length];
            reader.read_exact(&mut body_buf).unwrap();
            body = String::from_utf8_lossy(&body_buf).to_string();
        }
        
        // determining method, path, version
        let request_line: Vec<&str>= buffer[0].split_whitespace().collect(); 

        Self {
            method: request_line[0].to_string(), 
            path: request_line[1].to_string(), 
            version: request_line[2].to_string(), 
            headers, 
            body: Some(body)
        }
    }
}

struct Response {
    version: String,
    pub status: u32,
    pub status_description: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl Response { 
    pub fn new(version: String, status: u32, status_description: String, body: String) -> Self{ 
        Self { 
            version,
            status, 
            status_description, 
            headers: HashMap::new(), 
            body: Some(body)
        }
    }

    fn write_all(&self,mut stream: &TcpStream) -> std::io::Result<()>{ 
        let (content_length, content) = match &self.body { 
            Some(content) => (content.len(), content), 
            None => (0, &"".to_string())
        };
        let response_string = format!("{} {} {}\r\nContent-Type: text/html; charset=UTF-8\r\nContent-Length: {}\r\n\r\n{}", self.version, self.status, self.status_description, content_length, content);
        stream.write_all(response_string.as_bytes())
    }
}

fn router(
    request: &Request,
    routes: &HashMap<String, fn(&Request) -> Response>,
) -> Response {
    let k = format!("{}#{}", request.method, request.path);
    if routes.contains_key(&k) {
        let handler = routes.get(&k).unwrap();
        handler(request)
    } else {
        Response::new(request.version.to_string(),404, "Not Found".to_string(), "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<title>Not Found</title>\n</head>\n<body>\n<h1>404 Not Found</h1>\n<p>The requested resource was not found on this server.</p>\n</body>\n</html>\n".to_string())
    }
}

fn handle_root(request: &Request) -> Response {
    Response::new(request.version.to_string(), 200, "Ok".to_string(), "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<title>Hello World</title>\n</head>\n<body>\n<h1>Hello from the server!</h1>\n</body>\n</html>\n".to_string())
}

fn handle_about(request: &Request) -> Response {
    Response::new(request.version.to_string(), 200, "OK".to_string(), "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<title>About</title>\n</head>\n<body>\n<h1>About Page</h1>\n<p>This is a simple HTTP server implemented in Rust.</p>\n</body>\n</html>\n".to_string())
}

fn main() -> std::io::Result<()> {
    // open socket, bind and listen
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    println!("The server is up at 127.0.0.1:8000");

    // routing stuff
    let mut routes: HashMap<String, fn(&Request) -> Response> = HashMap::new();
    routes.insert("GET#/".to_string(), handle_root);
    routes.insert("GET#/about".to_string(), handle_about);
    let routes = Arc::new(routes); 

    let mut handles = vec![];
    // accept established connections
    for stream in listener.incoming() {
        // getting shared reference for the routes_map
        let routes_clone = Arc::clone(&routes);

        // creating a new thread
        let handle = thread::spawn( move || { 
            let stream = stream.unwrap(); 
    
            // reading + parsing request
            let request = Request::new(&stream); 
            // routing
            let response = router(&request, &routes_clone);

            // responding 
            response.write_all(&stream).unwrap();
        });
        handles.push(handle);
    }

    // wait for all the thread to complete execution
    for handle in handles { 
        handle.join().unwrap(); 
    }

    Ok(())
}
