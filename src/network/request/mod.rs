pub mod method;
pub mod request_path;

use super::headers::Headers;
use super::router::routes::Route;
use method::Method;
use request_path::RequestPath;

pub struct Request<T> {
    route: Route,
    path: RequestPath,
    pub headers: Headers,
    pub body: Option<T>,
}

impl<T> Request<T> {
    pub fn parse(request_string: &str) -> Result<Self, &'static str> {
        let lines: Vec<String> = request_string
            .lines()
            .map(|line| line.to_string())
            .collect();
        let first_line: Vec<String> = lines
            .first()
            .unwrap()
            .split(" ")
            .map(|data| data.to_string())
            .collect();

        let method: Method =
            Method::from_string(&first_line.get(0).unwrap()).expect("Invalid Method Type");
        let path: RequestPath = RequestPath::new(&first_line.get(1).unwrap());
        let route: Route = Route::new(path.get_route(), method);
        let mut headers: Headers = Headers::new(Some(&lines));
        let http_ver: String = first_line.get(2).unwrap().to_string();
        let body: Option<T> = None;

        println!(
            "method: {:?}\nfull-path: {}\nhttp_ver: {}\nquery_param: {:?}\nroute: {:?}",
            method,
            path.to_string(),
            http_ver,
            path.get_query_params(),
            path.get_route()
        );

        Ok(Request {
            route,
            path,
            headers,
            body,
        })
    }

    pub fn get_method(&self) -> Method {
        self.route.get_method()
    }

    pub fn get_path(&self) -> &RequestPath {
        &self.path
    }
    pub fn get_route(&self) -> &Route {
        &self.route
    }
}
