use self::main_controller::MainController;

use super::{
    super::{
        request::Request,
        response::{body::Body, Response},
    },
    routes::Route,
};

pub struct Controllers {
    pub controllers: Vec,
}

impl Controllers {
    pub fn new() -> Self {
        let mut controllers = Vec::new();
        controllers.push(Box::new(MainController));
        Controllers { controllers }
    }
}

pub trait Controller {
    fn handle_request(&self, request: Request<String>) -> Response<Body>;
    fn route_attrs(&self) -> Vec<Route>;
}

pub mod main_controller;
