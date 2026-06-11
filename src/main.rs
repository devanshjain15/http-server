use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;

// struct Request { }
// struct Response { }

fn parse_request_line(request: &str) -> (&str, &str, &str) {
    // method, path, version
    let line = request.lines().next().unwrap();
    let first_line: Vec<&str> = line.split_whitespace().collect();
    (first_line[0], first_line[1], first_line[2])
}

fn router(
    method: &str,
    path: &str,
    routes: &HashMap<String, fn(&str, &str) -> (u32, String, String)>,
) -> (u32, String, String) {
    let k = format!("{}#{}", method, path);
    if routes.contains_key(&k) {
        let handler = routes.get(&k).unwrap();
        handler(method, path)
    } else {
        (404, "Not Found".to_string(), "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<title>Not Found</title>\n</head>\n<body>\n<h1>404 Not Found</h1>\n<p>The requested resource was not found on this server.</p>\n</body>\n</html>\n".to_string())
    }
}

fn handle_root(_method: &str, _path: &str) -> (u32, String, String) {
    (200, "OK".to_string(), "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<title>Hello World</title>\n</head>\n<body>\n<h1>Hello from the server!</h1>\n</body>\n</html>\n".to_string())
}

fn handle_about(_method: &str, _path: &str) -> (u32, String, String) {
    (200, "OK".to_string(), "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<title>About</title>\n</head>\n<body>\n<h1>About Page</h1>\n<p>This is a simple HTTP server implemented in Rust.</p>\n</body>\n</html>\n".to_string())
}

fn main() -> std::io::Result<()> {
    // open socket, bind and listen
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    println!("The server is up at 127.0.0.1:8000");

    // routing stuff
    let mut routes: HashMap<String, fn(&str, &str) -> (u32, String, String)> = HashMap::new();
    routes.insert("GET#/".to_string(), handle_root);
    routes.insert("GET#/about".to_string(), handle_about);

    // accept established connections
    for stream in listener.incoming() {
        // copying and reading incoming raw bytes in chunks
        let mut reader = BufReader::new(stream.as_ref().unwrap());
        let mut buffer: Vec<String> = Vec::new();

        loop {
            let mut chunk = String::new();
            reader.read_line(&mut chunk).unwrap();
            if chunk == "\r\n" {
                break;
            }
            buffer.push(chunk.trim_end().to_string());
        }

        // parsing headers and checking for Content-Length
        let mut content_length = 0;
        for chunk in buffer.iter() {
            if chunk.contains("Content-Length:") {
                let header: Vec<&str> = chunk.split(' ').collect::<Vec<&str>>();
                content_length = header[1].parse().unwrap();
                break;
            }
        }
        println!("{}", content_length);

        if content_length != 0 {
            // find the end of the request based on the Content-Length
            let mut body_buf = vec![0; content_length];
            reader.read_exact(&mut body_buf).unwrap();
            buffer.push(String::from_utf8_lossy(&body_buf).into_owned());
        }

        let request = buffer.join("\n");
        println!("{request}");

        let (method, path, version) = parse_request_line(&request); 
        println!("{method}, {path}, {version}");

        let (status_code, status_text, body) = router(method, path, &routes);
        let content_len = body.len();

        // sending http response
        let status = format!("{version} {status_code} {status_text}");
        let response = format!(
            "{status}\r\nContent-Type: text/html; charset=UTF-8\r\nContent-Length: {content_len}\r\n\r\n{body}"
        );
        stream.unwrap().write_all(response.as_bytes())?;
    }

    Ok(())
}
