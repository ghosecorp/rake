---

# 🦀 Rake

**Rake** is a blazing-fast, beginner-friendly, zero-dependency **Rust HTTP server library** developed by **Ghosecorp**.

It enables you to spin up an HTTP server in just a few lines of Rust code, with support for:

- Custom route handling
- Static file serving (HTML/CSS/JS/images)
- Template engine support (strings & files)
- Request logging
- Multi-threaded request processing

---

## 🔰 Why Rake?

- 🛠️ **Minimal and lightweight** — uses only Rust's standard library!
- 💡 **Beginner-friendly** — learn web server fundamentals
- 📦 **Zero dependencies** — just clone and go!
- ✨ **Template support** — render HTML with placeholders
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
    
    // Register template engine
    server.set_template_engine(Arc::new(SimpleTemplateEngine));
    
    // Add routes
    server.route("GET", "/hello/", |_req, params| {
        let name = params.get("name").unwrap_or(&"World".into());
        Response::new(200, format!("Hello, {}!", name).into_bytes(), "text/plain")
    });
    
    // Serve static files
    server.static_dir("./static");
    
    server.start("127.0.0.1:7878");
}
```

---

## 🌟 Key Features

### ✅ Template Rendering
```rust
struct SimpleTemplateEngine; // Implements TemplateEngine trait

fn about_handler(_req: &Request, params: &HashMap<String, String>) -> Response {
    let mut context = HashMap::new();
    context.insert("username".into(), "Alice".into());
    
    let engine = SimpleTemplateEngine;
    let html = engine.render("templates/about.html", &context);
    Response::new(200, html.into_bytes(), "text/html")
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

Automatic console logging:

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

impl TemplateEngine for SimpleTemplateEngine {
    fn render(&self, path: &str, ctx: &HashMap<String, String>) -> String {
        let html = fs::read_to_string(path).unwrap_or_else(|_| 
            "Template error".into()
        );
        html.replace("{{ name }}", ctx.get("name").unwrap_or(&"Guest".into()))
    }
}

fn main() {
    let mut server = SimpleHttpServer::new();
    
    server.set_template_engine(Arc::new(SimpleTemplateEngine));
    
    // Template route
    server.route("GET", "/greet/", |_req, params| {
        let mut ctx = HashMap::new();
        ctx.insert("name".into(), params.get("name").cloned().unwrap_or_else(|| "Guest".into()));
        
        Response::new(
            200,
            server.template_engine.render("templates/greet.html", &ctx).into_bytes(),
            "text/html"
        )
    });
    
    // Static files
    server.static_dir("public");
    
    server.start("127.0.0.1:7878");
}
```

---

## 🛠 Project Structure

```
my_app/
├── public/          # Static files
│   └── style.css
├── templates/       # HTML templates
│   └── greet.html
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
- [The Rust Book (Chapter 20-21)](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)
- [Awesome Rust Web Servers](https://github.com/rust-unofficial/awesome-rust#web-servers)
- [Scaling Monolithic Rust Servers](https://www.reddit.com/r/rust/comments/zvt1mu/tips_on_scaling_a_monolithic_rust_web_server/)
- [YouTube - Rust Web Server Basics](https://www.youtube.com/watch?v=BHxmWTVFWxQ)

---