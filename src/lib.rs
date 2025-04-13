use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

type HandlerFn = fn(&Request) -> Response;

pub struct Request {
    pub method: String,
    pub path: String,
    pub raw: String,
}

pub struct Response {
    pub status_code: u16,
    pub body: String,
}

impl Response {
    pub fn to_http(&self) -> String {
        format!(
            "HTTP/1.1 {} OK\r\nContent-Type: text/plain\r\n\r\n{}",
            self.status_code, self.body
        )
    }
}

pub struct SimpleHttpServer {
    routes: HashMap<String, HandlerFn>,
}

impl SimpleHttpServer {
    pub fn new() -> Self {
        Self { 
            routes: HashMap::new()
        }
    }

    pub fn add_route(&mut self, path: &str, handler: HandlerFn) {
        self.routes.insert(path.to_string(), handler);
    }

    pub fn start(self, addr: &str) {
        let listener = TcpListener::bind(addr).expect("Failed to bind to address");
        println!("Listening on {}", addr);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let routes = self.routes.clone();
                    thread::spawn(move || {
                        handle_connection(stream, routes);
                    });
                }
                Err(e) => eprintln!("Connection failed: {}", e),
            }
        }
    }


}

fn handle_connection(mut stream: TcpStream, routes: HashMap<String, HandlerFn>) {
    let mut buffer = [0; 1024];

    if let Ok(_) = stream.read(&mut buffer) {
        let request_str = String::from_utf8_lossy(&buffer[..]).to_string();
        let lines: Vec<&str> = request_str.lines().collect();

        let (method, path) = if let Some(request_line) = lines.get(0) {
            let parts: Vec<&str> = request_line.split_whitespace().collect();
            if parts.len() >= 2 {
                (parts[0].to_string(), parts[1].to_string())
            } else {
                ("GET".to_string(), "/".to_string())
            }
        } else {
            ("GET".to_string(), "/".to_string())
        };

        let request = Request {
            method,
            path: path.clone(),
            raw: request_str,
        };

        let response = if let Some(handler) = routes.get(&path) {
            handler(&request)
        } else {
            Response {
                status_code: 404,
                body: "404 Not Found".to_string(),
            }
        };

        let http_response = response.to_http();
        stream.write_all(http_response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}