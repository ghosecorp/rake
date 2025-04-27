use crate::request::Request;
use crate::response::Response;
use std::collections::HashMap;

pub type HandlerFn = fn(&Request, &HashMap<String, String>) -> Response;

#[derive(Clone)]
pub struct Route {
    pub method: String,
    pub path: String,
    pub handler: HandlerFn,
}

#[derive(Clone)]
pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    pub fn add_route(&mut self, method: &str, path: &str, handler: HandlerFn) {
        self.routes.push(Route {
            method: method.to_uppercase(),
            path: path.to_string(),
            handler,
        });
    }

    pub fn find(&self, method: &str, path: &str) -> Option<(&HandlerFn, HashMap<String, String>)> {
        for route in &self.routes {
            if route.method == method.to_uppercase() {
                if let Some(params) = match_route(&route.path, path) {
                    return Some((&route.handler, params));
                }
            }
        }
        None
    }
}

// Match dynamic routes like /hello/<name>
fn match_route(pattern: &str, path: &str) -> Option<HashMap<String, String>> {
    let mut params = HashMap::new();
    let pat_parts: Vec<_> = pattern.trim_matches('/').split('/').collect();
    let path_parts: Vec<_> = path.trim_matches('/').split('/').collect();
    if pat_parts.len() != path_parts.len() {
        return None;
    }
    for (pat, val) in pat_parts.iter().zip(path_parts.iter()) {
        if pat.starts_with('<') && pat.ends_with('>') {
            params.insert(pat[1..pat.len()-1].to_string(), val.to_string());
        } else if pat != val {
            return None;
        }
    }
    Some(params)
}