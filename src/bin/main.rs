use rake::{SimpleHttpServer, Request, Response};

fn route_handler(_req: &Request) -> Response {
    Response {
        status_code: 200,
        body: "This is route".to_string(),
        content_type: "text/plain".to_string(),
    }
}

fn hello_handler(_req: &Request) -> Response {
    Response {
        status_code: 200,
        body: "Hello from /hello".to_string(),
        content_type: "text/plain".to_string(),
    }
}

fn about_handler(_req: &Request) -> Response {
    Response {
        status_code: 200,
        body: "This is a custom Rust HTTP server.".to_string(),
        content_type: "text/plain".to_string(),
    }
}


fn main() {
    let mut server = SimpleHttpServer::new();

    // Dynamic routes
    server.add_route("/", route_handler);
    server.add_route("/hello", hello_handler);
    server.add_route("/about", about_handler);

    // Static file serving from ./public when URL starts with /static/
    server.add_static_route("/static/", "public");
    

    server.start("127.0.0.1:7878");
}