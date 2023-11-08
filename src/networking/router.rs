use std::collections::HashMap;

use super::{{ request::method::Method, Request }, response::Response, body::Body};

pub struct Router {
    routes: HashMap<(Method, String), fn(Request<String>) -> Response<Body>>,
}

impl Router {
    pub fn new() -> Self {
        Router { routes: HashMap::new() }
    }

    pub fn add_route(&mut self, method: Method, path: &str, handler: fn(Request<String>) -> Response<Body>) {
        self.routes.insert((method, path.to_string()), handler);
    }

    pub fn route_request(&self, request: Request<String>) -> Response<Body> {
        if let Some(handler) = self.routes.get(&(request.get_method(), request.get_path().to_string())) {
            handler(request)
        } else {
            Response::not_found(Body::from_str("Page Not found"), request.headers)
        }
    }
}
