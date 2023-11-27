use crate::network::{
    headers::Headers,
    request::{method::Method, Request},
    response::{body::Body, Response},
    router::{routes::Route, Controller},
};
use std::fs;

pub struct MainController;

impl Controller for MainController {
    fn handle_request(&self, request: Request<String>) -> Response<Body> {
        let route: &Route = request.get_route();
        match (route.get_method(), route.get_path()) {
            (Method::GET, "/") => self.main_page(),
            _ => Response::not_found(Body::from_str("Page Not found"), request.headers),
        }
    }

    fn route_attrs(&self) -> Vec<Route> {
        vec![Route {
            path: "/",
            method: Method::GET,
        }]
    }
}

impl MainController {
    pub fn new() -> Self {
        MainController
    }

    pub fn main_page(&self) -> Response<Body> {
        let mut headers = Headers::new(None);
        headers.add_header("Content-Type", "text/html");
        let response = match fs::read_to_string("static/index.html") {
            Ok(file) => Response::ok(Body::new(file), headers),
            Err(err) => Response::internal_server_error(
                Body::new(format!("500: File Read error: {:?}", err)),
                headers,
            ),
        };
        response
    }
}
