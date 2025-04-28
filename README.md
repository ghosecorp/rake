# 🦀 Rake

**Rake** is a blazing-fast, beginner-friendly, zero-dependency **Rust HTTP server library** developed by **Ghosecorp**.

It enables you to spin up an HTTP server in just a few lines of Rust code, with support for:

- Custom route handling
- Static file serving (HTML/CSS/JS/images)
- Template engine support (both strings & files)
- Request logging
- Multi-threaded request processing

---

## 🔰 Why Rake?

- 🛠️ **Minimal and lightweight** — uses only Rust's standard library!
- 💡 **Beginner-friendly** — learn web server fundamentals
- 📦 **Zero dependencies** — just clone and go!
- ✨ **Template support** — render HTML templates from **strings or files**
- 📜 **Request logging** — built-in debug logging
- 🔐 **Secure sessions** — cookie-based session handling
- ✍️ **Created by**: [Ghosecorp](https://github.com/Ghosecorp)

---

## 🚀 Quick Start

### 1. Create Project & Add Rake
```bash
cargo new my_app
cd my_app
```

### 2. Basic Server (`src/main.rs`)
```rust
use rake::{SimpleHttpServer, Request, Response, TemplateEngine};
use std::collections::HashMap;
use std::sync::Arc;

struct SimpleTemplateEngine;

impl TemplateEngine for SimpleTemplateEngine {
    fn render(&self, template: &str, context: &HashMap<String, String>) -> String {
        let mut result = template.to_string();
        for (key, value) in context {
            result = result.replace(&format!("{{{{{}}}}}", key), value);
        }
        result
    }
}

fn main() {
    let mut server = SimpleHttpServer::new();
    
    server.set_template_engine(Arc::new(SimpleTemplateEngine));
    
    server.route("GET", "/hello/", |_req, params| {
        let name = params.get("name").unwrap_or(&"World".into());
        Response::new(200, format!("Hello, {}!", name).into_bytes(), "text/plain")
    });

    server.static_dir("./static");

    server.start("127.0.0.1:7878");
}
```

---

## 🌟 Key Features

### ✅ Template Rendering

Templates can be rendered **from files** or **directly from string literals**!

---

### 🔥 Using Template from a String
If you don't want to create a separate HTML file, you can use inline templates:

```rust
use rake::{SimpleHttpServer, Request, Response, TemplateEngine};
use std::collections::HashMap;
use std::sync::Arc;

struct SimpleTemplateEngine;

impl SimpleTemplateEngine {
    pub fn new() -> Self {
        SimpleTemplateEngine
    }
}

impl TemplateEngine for SimpleTemplateEngine {
    fn render(&self, template: &str, context: &HashMap<String, String>) -> String {
        let mut result = template.to_string();
        for (key, value) in context {
            let placeholder1 = format!("{{{{ {} }}}}", key);
            let placeholder2 = format!("{{{{{}}}}}", key);
            result = result.replace(&placeholder1, value);
            result = result.replace(&placeholder2, value);
        }
        result
    }
}

fn template_string_hello_handler(_req: &Request, params: &HashMap<String, String>) -> Response {
    let engine = SimpleTemplateEngine::new();

    let template = "<html><body>Hello, {{ name }}!</body></html>";
    let name = params.get("name").cloned().unwrap_or_else(|| "World".to_string());

    let mut context = HashMap::new();
    context.insert("name".to_string(), name);

    let rendered = engine.render(template, &context);
    Response::new(200, rendered.into_bytes(), "text/html")
}
```

---

### 🔥 Using Template from a File

If you prefer managing HTML separately:

```rust
fn template_file_hello_handler(_req: &Request, params: &HashMap<String, String>) -> Response {
    let mut context = HashMap::new();
    let name = params.get("name").cloned().unwrap_or_else(|| "World".to_string());
    context.insert("name".to_string(), name);

    let engine = SimpleTemplateEngine::new();
    let rendered = engine.render("public/hello.html", &context);

    Response::new(200, rendered.into_bytes(), "text/html")
}
```

---

### ✅ Static File Serving

```
project-root/
├── static/
│   ├── style.css
│   └── logo.png
```

Access via:

- `http://localhost:7878/style.css`
- `http://localhost:7878/logo.png`

---

### ✅ Request Logging

Automatic console output:

```text
[GET] Request: /about => Status: 200
[GET] Request: /assets/image.jpg => Status: 404
```

---

## 📚 Full Example

```rust
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
    fn render(&self, template: &str, context: &HashMap<String, String>) -> String {
        let mut result = template.to_string();
        for (key, value) in context {
            let placeholder1 = format!("{{{{ {} }}}}", key);
            let placeholder2 = format!("{{{{{}}}}}", key);
            result = result.replace(&placeholder1, value);
            result = result.replace(&placeholder2, value);
        }
        result
    }
}

fn hello_handler(_req: &Request, params: &HashMap<String, String>) -> Response {
    let default = "World".to_string();
    let name = params.get("name").unwrap_or(&default);
    let body = format!("Hello, {}!", name);
    Response::new(200, body.into_bytes(), "text/plain")
}

fn template_file_about_handler(_req: &Request, _params: &HashMap<String, String>) -> Response {
    match fs::read("public/about.html") {
        Ok(contents) => Response::new(200, contents, "text/html"),
        Err(_) => Response::new(404, b"File not found".to_vec(), "text/plain"),
    }
}

fn template_string_hello_handler(_req: &Request, params: &HashMap<String, String>) -> Response {
    let engine = SimpleTemplateEngine::new();
    let template = "<html><body>Hello, {{ name }}!</body></html>";

    let mut context = HashMap::new();
    let name = params.get("name").cloned().unwrap_or_else(|| "World".to_string());
    context.insert("name".to_string(), name);

    let rendered = engine.render(template, &context);
    Response::new(200, rendered.into_bytes(), "text/html")
}

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
    server.route("GET", "/about/", template_file_about_handler);
    server.route("GET", "/hello-template-string/<name>", template_string_hello_handler);
    server.route("GET", "/hello-template-file/<name>", template_file_hello_handler);

    server.static_dir("./public");

    server.start("127.0.0.1:7878");
}
```

---

## 🛠 Project Structure

```
my_app/
├── public/          # Static files & HTML templates
│   ├── hello.html
│   ├── about.html
│   └── style.css
├── src/
│   └── main.rs
└── Cargo.toml
```

---

## 📜 License & Contributing

- **License**: Apache 2.0
- **Contributions**: Welcome! See [CONTRIBUTING.md](CONTRIBUTING.md)
- **Created by**: [Ghosecorp](https://github.com/Ghosecorp)

---

## 📖 Citations

- [The Rust Book (Chapters 20-21)](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)
- [Awesome Rust Web Servers](https://github.com/rust-unofficial/awesome-rust#web-servers)
- [Scaling Monolithic Rust Servers (Reddit)](https://www.reddit.com/r/rust/comments/zvt1mu/tips_on_scaling_a_monolithic_rust_web_server/)
- [YouTube - Rust Web Server Basics](https://www.youtube.com/watch?v=BHxmWTVFWxQ)

---