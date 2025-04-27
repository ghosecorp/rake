use std::collections::HashMap;

pub struct Response {
    pub status_code: u16,
    pub body: Vec<u8>,
    pub content_type: String,
    pub headers: HashMap<String, String>,
}

impl Response {
    pub fn new(status_code: u16, body: Vec<u8>, content_type: &str) -> Self {
        Self {
            status_code,
            body,
            content_type: content_type.to_string(),
            headers: HashMap::new(),
        }
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn to_http(&self) -> Vec<u8> {
        let mut header = format!(
            "HTTP/1.1 {} OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n",
            self.status_code,
            self.content_type,
            self.body.len()
        );
        for (k, v) in &self.headers {
            header.push_str(&format!("{}: {}\r\n", k, v));
        }
        header.push_str("\r\n");
        let mut response = header.into_bytes();
        response.extend(&self.body);
        response
    }
}
