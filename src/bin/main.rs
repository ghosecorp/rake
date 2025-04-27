use rake::{SimpleHttpServer, Request, Response, TemplateEngine};
use std::collections::HashMap;
use std::sync::Arc;
use std::fs;

struct SimpleTemplateEngine;

impl SimpleTemplateEngine {
    pub fn new() -> Self {
        SimpleTemplateEngine
    }
}

impl TemplateEngine for SimpleTemplateEngine {
    // Render from a file path
    fn render(&self, template_path: &str, context: &HashMap<String, String>) -> String {
        let html = fs::read_to_string(template_path)
            .unwrap_or_else(|_| "<h1>Template not found</h1>".to_string());
        self.render_str(&html, context)
    }

    // Render from a template string directly
    fn render_str(&self, template: &str, context: &HashMap<String, String>) -> String {
        let mut result = template.to_string();
        for (key, value) in context {
            // Replace both {{ name }} and {{name}}
            let placeholder1 = format!("{{{{ {} }}}}", key);
            let placeholder2 = format!("{{{{{}}}}}", key);
            result = result.replace(&placeholder1, value);
            result = result.replace(&placeholder2, value);
        }
        result
    }
}

// Handler for /hello/<name>
fn hello_handler(_req: &Request, params: &HashMap<String, String>) -> Response {
    let default = "world".to_string();
    let name = params.get("name").unwrap_or(&default);
    let body = format!("Hello, {}!", name);
    Response::new(200, body.into_bytes(), "text/plain")
}

// Handler for /echo (POST)
fn echo_handler(req: &Request, _params: &HashMap<String, String>) -> Response {
    Response::new(200, req.body.clone(), "text/plain")
}

// Handler for /hello-template-string/<name>
fn template_string_hello_handler(_req: &Request, params: &HashMap<String, String>) -> Response {
    let engine = SimpleTemplateEngine::new();

    // Both {{ name }} and {{name}} will work
    let template = "<html><body>Hello, {{ name }}!</body></html>";
    let name = params.get("name").cloned().unwrap_or_else(|| "world".to_string());

    let mut context = HashMap::new();
    context.insert("name".to_string(), name);

    let rendered = engine.render_str(template, &context);
    Response::new(200, rendered.into_bytes(), "text/html")
}

// Handler for /hello-template-file/<name>
fn template_file_hello_handler(_req: &Request, params: &HashMap<String, String>) -> Response {
    let mut context = HashMap::new();
    let name = params.get("name").cloned().unwrap_or_else(|| "World".to_string());
    context.insert("name".to_string(), name);

    let engine = SimpleTemplateEngine::new();
    let rendered = engine.render("public/hello.html", &context);

    Response::new(200, rendered.into_bytes(), "text/html")
}

fn main() {
    let mut server = SimpleHttpServer::new();

    server.set_template_engine(Arc::new(SimpleTemplateEngine::new()));

    server.route("GET", "/hello/<name>", hello_handler);
    server.route("POST", "/echo", echo_handler);

    // Serve static files from ./static directory
    server.static_dir("./static");

    server.route("GET", "/hello-template-string/<name>", template_string_hello_handler);
    server.route("GET", "/hello-template-file/<name>", template_file_hello_handler);

    server.start("127.0.0.1:7878");
}