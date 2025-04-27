use crate::request::Request;
use crate::response::Response;
use crate::router::{Router, HandlerFn};
use crate::template::TemplateEngine;

use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::path::Path;
use std::fs;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

pub type BeforeMiddleware = fn(&mut Request) -> Option<Response>;
pub type AfterMiddleware = fn(&Request, &mut Response);
pub type ErrorHandlerFn = fn(&Request, u16) -> Response;

static SESSION_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct SimpleHttpServer {
    router: Router,
    error_handlers: HashMap<u16, ErrorHandlerFn>,
    pub static_dir: Option<String>,
    sessions: Arc<Mutex<HashMap<String, HashMap<String, String>>>>,
    template_engine: Option<Arc<dyn TemplateEngine>>,
    before_middlewares: Vec<BeforeMiddleware>,
    after_middlewares: Vec<AfterMiddleware>,
}

impl SimpleHttpServer {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
            error_handlers: HashMap::new(),
            static_dir: None,
            sessions: Arc::new(Mutex::new(HashMap::new())),
            template_engine: None,
            before_middlewares: Vec::new(),
            after_middlewares: Vec::new(),
        }
    }

    pub fn route(&mut self, method: &str, path: &str, handler: HandlerFn) {
        self.router.add_route(method, path, handler);
    }

    pub fn static_dir(&mut self, dir: &str) {
        self.static_dir = Some(dir.to_string());
    }

    pub fn error_handler(&mut self, code: u16, handler: ErrorHandlerFn) {
        self.error_handlers.insert(code, handler);
    }

    pub fn set_template_engine(&mut self, engine: Arc<dyn TemplateEngine>) {
        self.template_engine = Some(engine);
    }

    pub fn add_before_middleware(&mut self, mw: BeforeMiddleware) {
        self.before_middlewares.push(mw);
    }

    pub fn add_after_middleware(&mut self, mw: AfterMiddleware) {
        self.after_middlewares.push(mw);
    }

    pub fn start(&self, addr: &str) {
        let listener = TcpListener::bind(addr).expect("Failed to bind to address");
        println!("Listening on {}", addr);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let router = self.router.clone();
                    let error_handlers = self.error_handlers.clone();
                    let static_dir = self.static_dir.clone();
                    let sessions = Arc::clone(&self.sessions);
                    let template_engine = self.template_engine.clone();
                    let before_middlewares = self.before_middlewares.clone();
                    let after_middlewares = self.after_middlewares.clone();

                    thread::spawn(move || {
                        handle_connection(
                            stream,
                            router,
                            error_handlers,
                            static_dir,
                            sessions,
                            template_engine,
                            before_middlewares,
                            after_middlewares,
                        );
                    });
                }
                Err(e) => eprintln!("Connection failed: {}", e),
            }
        }
    }
}

fn generate_session_id() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let count = SESSION_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{:x}{:x}", now, count)
}

fn handle_connection(
    mut stream: TcpStream,
    router: Router,
    error_handlers: HashMap<u16, ErrorHandlerFn>,
    static_dir: Option<String>,
    sessions: Arc<Mutex<HashMap<String, HashMap<String, String>>>>,
    _template_engine: Option<Arc<dyn TemplateEngine>>,  // unused, prefixed with _
    before_middlewares: Vec<BeforeMiddleware>,
    after_middlewares: Vec<AfterMiddleware>,
) {
    let mut buffer = [0; 8192];
    if let Ok(size) = stream.read(&mut buffer) {
        let request_str = String::from_utf8_lossy(&buffer[..size]).to_string();
        let (method, path, headers, body, query) = parse_http_request(&request_str);

        let mut request = Request {
            method: method.clone(),
            path: path.clone(),
            raw: request_str,
            headers,
            query,
            body,
        };

        // Run before middlewares
        for mw in &before_middlewares {
            if let Some(resp) = mw(&mut request) {
                send_response(&mut stream, resp);
                return;
            }
        }

        // Session handling
        let mut session_id = None;
        if let Some(cookie_header) = request.headers.get("cookie") {
            for cookie in cookie_header.split(';') {
                let cookie = cookie.trim();
                if let Some((k, v)) = cookie.split_once('=') {
                    if k == "SESSIONID" {
                        session_id = Some(v.to_string());
                    }
                }
            }
        }
        let session_id = session_id.unwrap_or_else(generate_session_id);

        let mut sessions_lock = sessions.lock().unwrap();
        let _session_data = sessions_lock.entry(session_id.clone()).or_insert_with(HashMap::new);
        drop(sessions_lock); // release lock early

        // Match route
        let response = if let Some((handler, params)) = router.find(&method, &path) {
            handler(&request, &params)
        } else if let Some(dir) = &static_dir {
            // Serve static files
            let full_path = Path::new(dir).join(path.trim_start_matches('/'));
            match fs::read(&full_path) {
                Ok(contents) => {
                    let content_type = get_mime_type(&full_path);
                    Response::new(200, contents, content_type)
                }
                Err(_) => error_response(404, &request, &error_handlers),
            }
        } else {
            error_response(404, &request, &error_handlers)
        };

        let mut response = response.with_header("Set-Cookie", &format!("SESSIONID={}; HttpOnly; Path=/", session_id));

        // Logs 
        println!(
            "[{}] Request: {} => Status: {}",
            method,
            path,
            response.status_code
        );

        // Run after middlewares
        for mw in &after_middlewares {
            mw(&request, &mut response);
        }

        send_response(&mut stream, response);
    }
}

fn send_response(stream: &mut TcpStream, response: Response) {
    let http_response = response.to_http();
    let _ = stream.write_all(&http_response);
    let _ = stream.flush();
}

fn error_response(code: u16, req: &Request, handlers: &HashMap<u16, ErrorHandlerFn>) -> Response {
    if let Some(handler) = handlers.get(&code) {
        handler(req, code)
    } else {
        Response::new(code, format!("{} Error", code).into_bytes(), "text/plain")
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
        _ => "application/octet-stream",
    }
}

fn parse_http_request(raw: &str) -> (String, String, HashMap<String, String>, Vec<u8>, HashMap<String, String>) {
    let mut lines = raw.lines();
    let request_line = lines.next().unwrap_or("");
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("GET").to_string();
    let mut path = parts.next().unwrap_or("/").to_string();

    let mut query = HashMap::new();
    if let Some(pos) = path.find('?') {
        let path_clone = path.clone();
        let q = &path_clone[pos+1..];
        path = path_clone[..pos].to_string();
        for kv in q.split('&') {
            let mut iter = kv.splitn(2, '=');
            if let (Some(k), Some(v)) = (iter.next(), iter.next()) {
                query.insert(k.to_string(), v.to_string());
            }
        }
    }

    let mut headers = HashMap::new();
    let mut body = Vec::new();
    let mut in_body = false;
    for line in lines {
        if in_body {
            body.extend_from_slice(line.as_bytes());
            body.push(b'\n');
        } else if line.is_empty() {
            in_body = true;
        } else if let Some((k, v)) = line.split_once(':') {
            headers.insert(k.trim().to_ascii_lowercase(), v.trim().to_string());
        }
    }

    (method, path, headers, body, query)
}
