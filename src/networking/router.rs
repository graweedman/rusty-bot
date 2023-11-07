use std::collections::HashMap;

use super::{request::{Method, Request}, response::Response};

pub struct Router {
    routes: HashMap<(Method, String), fn(Request<String>) -> Response<String>>,
}

impl Router {
    pub fn add_route(&mut self, method: Method, path: &str, handler: fn(Request<String>) -> Response<String>) {
        self.routes.insert((method, path.to_string()), handler);
    }

    pub fn route_request(&self, request: Request<String>) -> Response<String> {
        if let Some(handler) = self.routes.get(&(request.get_method(), request.get_path().to_string())) {
            handler(request)
        } else {
            Response::not_found("Page not Found".to_string(), request.headers)
        }
    }
}
