use rake::{SimpleHttpServer, Request, Response};

fn route_handler(_req: &Request) -> Response {
    Response {
        status_code: 200,
        body: "This is route".to_string().into_bytes(),
        content_type: "text/plain".to_string(),
    }
}

fn hello_handler(_req: &Request) -> Response {
    Response {
        status_code: 200,
        body: "Hello from /hello".to_string().into_bytes(),
        content_type: "text/plain".to_string(),
    }
}

fn about_handler(_req: &Request) -> Response {
    Response {
        status_code: 200,
        body: "This is a custom Rust HTTP server.".to_string().into_bytes(),
        content_type: "text/plain".to_string(),
    }
}

fn json_handler(_req: &Request) -> Response {
    // Manually creating a JSON response body
    let json_body = r#"{
        "content": "hi"
    }"#;

    Response {
        status_code: 200,
        body: json_body.to_string().into_bytes(),
        content_type: "application/json".to_string(),
    }
}


fn main() {
    let mut server = SimpleHttpServer::new();

    // Dynamic routes
    server.add_route("/", route_handler);
    server.add_route("/hello", hello_handler);
    server.add_route("/about", about_handler);

    // Add route for JSON response
    server.add_route("/json", json_handler);


    // Static file serving from ./public when URL starts with /static/
    server.add_static_route("/static/", "public");
    server.add_static_route("/assets/", "public/assets");

    server.add_file_route("/home", "public/index.html");
    server.add_file_route("/about", "public/about.html");
    server.add_file_route("/testingabout", "/home/edwardsepiol/Projects/Rust_Web/rake/public/about.html");

    server.start("127.0.0.1:7878");
}