use std::collections::HashMap;

pub struct Request {
    pub method: String,
    pub path: String,
    pub raw: String,
    pub headers: HashMap<String, String>,
    pub query: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Request {
    pub fn header(&self, key: &str) -> Option<&String> {
        self.headers.get(&key.to_ascii_lowercase())
    }

    pub fn param(&self, key: &str) -> Option<&String> {
        self.query.get(key)
    }

    // Parse URL query parameters
    pub fn query_params(&self) -> HashMap<String, String> {
        self.query.clone()
    }

    // Parse URL-encoded form data from body
    pub fn form_data(&self) -> HashMap<String, String> {
        parse_urlencoded(&self.body)
    }
}

// Helper to parse URL-encoded data
fn parse_urlencoded(data: &[u8]) -> HashMap<String, String> {
    let s = String::from_utf8_lossy(data);
    s.split('&').filter_map(|pair| {
        let mut parts = pair.splitn(2, '=');
        match (parts.next(), parts.next()) {
            (Some(k), Some(v)) => Some((k.to_string(), v.to_string())),
            _ => None,
        }
    }).collect()
}