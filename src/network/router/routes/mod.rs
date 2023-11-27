use crate::network::request::method::Method;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Route {
    pub path: &'static str,
    pub method: Method,
}

impl Route {
    pub fn new(path: &str, method: Method) -> Self {
        Route { path, method }
    }

    pub fn get_method(&self) -> Method {
        self.method
    }

    pub fn get_path(&self) -> &str {
        self.path
    }
}
