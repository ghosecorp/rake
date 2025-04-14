use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::path::Path;
use std::fs;
type HandlerFn = fn(&Request) -> Response;

pub struct Request {
    pub method: String,
    pub path: String,
    pub raw: String,
}

pub struct Response {
    pub status_code: u16,
    pub body: Vec<u8>,
    pub content_type: String,
}

impl Response {
    pub fn to_http(&self) -> Vec<u8> {
        let header = format!(
            "HTTP/1.1 {} OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
            self.status_code,
            self.content_type,
            self.body.len()
        );

        let mut response = header.into_bytes();
        response.extend(&self.body);
        response
    }
}


fn get_mime_type(path: &Path) -> &str {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("html") => "text/html",
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("svg") => "image/svg+xml",
        Some("json") => "application/json",
        Some("pdf") => "application/pdf",
        Some("txt") => "text/plain",
        _ => "application/octet-stream", // default for unknown types
    }
}


#[derive(Clone)]
pub struct StaticRoute {
    pub route_prefix: String,
    pub directory: String,
}

pub struct SimpleHttpServer {
    routes: HashMap<String, HandlerFn>,
    static_routes: Vec<StaticRoute>,
}

impl SimpleHttpServer {
    pub fn new() -> Self {
        Self { 
            routes: HashMap::new(),
            static_routes: Vec::new(),
        }
    }

    pub fn add_route(&mut self, path: &str, handler: HandlerFn) {
        self.routes.insert(path.to_string(), handler);
    }

    pub fn add_static_route(&mut self, route_prefix: &str, directory: &str) {
        self.static_routes.push(StaticRoute {
            route_prefix: route_prefix.to_string(),
            directory: directory.to_string(),
        });
    }

    pub fn start(&self, addr: &str) {
        let listener = TcpListener::bind(addr).expect("Failed to bind to address");
        println!("Listening on {}", addr);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let routes = self.routes.clone();
                    let static_routes = self.static_routes.clone();
                    thread::spawn(move || {
                        handle_connection(stream, routes, static_routes);
                    });
                }
                Err(e) => eprintln!("Connection failed: {}", e),
            }
        }
    }
}


fn handle_connection(
    mut stream: TcpStream,
    routes: HashMap<String, HandlerFn>,
    static_routes: Vec<StaticRoute>,
) {
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

        // Match static route
        let response = if let Some(static_route) = match_static_route(&path, &static_routes) {
            serve_static_file(&path, static_route)
        } else if let Some(handler) = routes.get(&path) {
            handler(&request)
        } else {
            Response {
                status_code: 404,
                body: "404 Not Found".to_string().into_bytes(),
                content_type: "text/plain".to_string(),
            }
        };

        let http_response = response.to_http();
        stream.write_all(&http_response).unwrap();
        stream.flush().unwrap()

    }
}

fn match_static_route<'a>(path: &str, static_routes: &'a [StaticRoute]) -> Option<&'a StaticRoute> {
    static_routes
        .iter()
        .find(|route| path.starts_with(&route.route_prefix))
}

fn serve_static_file(path: &str, static_route: &StaticRoute) -> Response {
    let rel_path = path.trim_start_matches(&static_route.route_prefix);
    let full_path = Path::new(&static_route.directory).join(rel_path);

    match fs::read(&full_path) {
        Ok(contents) => {
            let content_type = get_mime_type(&full_path).to_string();
            Response {
                status_code: 200,
                body: contents,
                content_type,
            }
        }
        Err(_) => Response {
            status_code: 404,
            body: b"404 File Not Found".to_vec(),
            content_type: "text/plain".to_string(),
        },
    }
}
