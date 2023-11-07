use super::headers::Headers;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE
}

pub struct Request<T> {
    method: String,
    path: String,
    pub headers: Headers,
    pub body: Option<T>,
    // meta_data: String,
    // body: String
}

impl<T> Request<T> {

    pub fn parse(request_string: &str) -> Result<Self, &'static str> {

        let lines: Vec<String> = request_string.lines().map(|line| line.to_string()).collect();
        let first_line: Vec<String> = lines.first().unwrap().split(" ").map(|data| data.to_string()).collect();

        let method: String = first_line.first().unwrap().to_string();
        let path: String = first_line.get(1).unwrap().to_string();
        let http_ver: String = first_line.get(2).unwrap().to_string();
        let body: Option<T> = None;
        let mut headers: Headers = Headers::new();

        println!(" method: {:?}\n path: {}\n http_ver: {}", method, path, http_ver);

        for line in &lines[1..] {
            if line.is_empty() {
                break;
            }
            let mut header_parts = line.splitn(2, ": ");
            if let (Some(key), Some(value)) = (header_parts.next(), header_parts.next()) {
                headers.add_header(key, value);
            }
        }

        Ok( Request{ method , path, headers, body} )
    }

    pub fn get_method(&self) -> Method {
        match self.method.as_str() {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            _ => panic!("Invalid Method")
        }
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }
}
