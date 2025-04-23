# 🦀 Rake

**Rake** is a blazing-fast, beginner-friendly, zero-dependency **Rust HTTP server library** crafted with ❤️ by **Ghosecorp**.
...

It enables you to spin up an HTTP server in just a few lines of Rust code — with support for:

- Custom route handling
- Static file serving (like HTML, CSS, JS, images, etc.)
- Multi-threaded request processing

---

## 🔰 Why Rake?

- 🛠️ **Minimal and lightweight** — uses only Rust’s standard library!
- 💡 **Beginner-friendly** — built to help you learn how web servers work.
- 🔐 **Zero dependencies** — just clone and go!
- ✍️ **Created by**: [Ghosecorp](https://github.com/Ghosecorp)

---

## 🚀 Simple Rust HTTP Server

This is a super simple HTTP server written in **pure Rust (standard library only)**. It supports:

- ✨ Custom route handlers
- 📁 Static file serving (images, HTML, etc.)
- 📡 Multi-threaded request handling

---

## 📦 Features

- ✅ Route different paths to different handler functions
- 🖼️ Serve static files from a directory (like `/public`)
- 🧵 Handle multiple client requests using threads

---

## 👶 For Beginners: How to Use Rake in Your Rust Project

### Step 1: Create a New Project

```bash
cargo new my_rake_project
cd my_rake_project
```

---

### Step 2: Add Rake Library

If you’ve cloned the `rake` project locally, include it in your `Cargo.toml`:

```toml
[dependencies]
rake = { path = "../path_to_rake" }
```

Or if it's published on crates.io in the future:

```toml
[dependencies]
rake = "0.1"
```

---

### Step 3: Use Rake in `main.rs`

```rust
use rake::{SimpleHttpServer, Request, Response};

fn home_handler(_req: &Request) -> Response {
    Response {
        status_code: 200,
        body: "<h1>Welcome to the homepage!</h1>".as_bytes().to_vec(),
        content_type: "text/html".to_string(),
    }
}

fn hello_handler(_req: &Request) -> Response {
    Response {
        status_code: 200,
        body: "Hello from /hello".as_bytes().to_vec(),
        content_type: "text/plain".to_string(),
    }
}

fn main() {
    let mut server = SimpleHttpServer::new();

    // Add routes
    server.add_route("/", home_handler);
    server.add_route("/hello", hello_handler);

    // Add static file serving
    server.add_static_route("/static", "public");

    // Start the server
    server.start("127.0.0.1:7878");
}
```

---

### Step 4: Add Some Static Files

Create a directory for static assets (e.g. `public/`) and add files like `index.html`, images, etc.

```
my_rake_project/
├── public/
│   └── image.jpg
├── src/
│   └── main.rs
```

---

### Step 5: Run the Server

```bash
cargo run
```

Now open your browser and try:

- 🏠 [http://localhost:7878/](http://localhost:7878/)
- 👋 [http://localhost:7878/hello](http://localhost:7878/hello)
- 🖼️ [http://localhost:7878/static/image.jpg](http://localhost:7878/static/image.jpg)

---

## 💡 How it Works

- It listens for HTTP requests on a socket using `TcpListener`.
- It parses the raw request, maps routes to handlers or static files.
- MIME types are automatically inferred from file extensions.
- The response is manually constructed and written back to the socket.

---

## 🔐 No Dependencies

This server **only uses**:

- `std::net`
- `std::thread`
- `std::fs`, `std::path`
- `std::collections`
- `std::io`

---

## 🛠️ TODO

- [ ] Add support for `POST` and request bodies
- [ ] Logging with timestamps
- [ ] Templating engine support
- [ ] HTTPS with TLS (future)

---

## 📜 License

Licensed under the [Apache Version 2.0 License](LICENSE).

---

## 🙌 Contributing

Pull requests are welcome! Feel free to fork and submit ideas or improvements.

---

## 📢 Created by [Ghosecorp](https://github.com/Ghosecorp)
```