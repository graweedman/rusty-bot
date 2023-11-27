use std::collections::HashMap;

pub mod controllers;
pub mod routes;

use crate::network::{
    request::Request,
    response::{body::Body, Response},
    router::{controllers::Controller, routes::Route},
};

pub struct Router {
    routes: HashMap<Route, &'static (dyn Controller + 'static)>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }

    fn add_routes<T: 'static + Controller>(&mut self, controller: &'static T) {
        for route_attr in controller.route_attrs() {
            self.routes.insert(route_attr, controller);
        }
    }

    pub fn route_request(&self, request: Request<String>) -> Response<Body> {
        if let Some(handler) = self.routes.get(request.get_route()) {
            handler.handle_request(request)
        } else {
            Response::not_found(Body::from_str("Page Not found"), request.headers)
        }
    }
}
