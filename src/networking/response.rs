use super::headers::Headers;

mod http_status;



use http_status::HttpStatus;
use super::body::{Body, BodyBuffer};

pub struct Response<T> {
    pub status: HttpStatus,
    pub headers: Headers,
    pub body: Option<T>,
}

impl Response<Body> {

    pub fn new(status: HttpStatus, body: Option<Body>, headers:Headers ) -> Self {
        Response { status, headers, body }
    }

    pub fn ok(body: Body, headers: Headers) -> Self {
        let status = HttpStatus::Ok;
        let body = Some(body);

        Response { status, headers, body }
    }

    pub fn not_found(body: Body, headers: Headers) -> Self {
        let status = HttpStatus::NotFound;
        let body= Some(body);

        Response {status , headers, body }
    }

    pub fn internal_server_error(body: Body, headers: Headers) -> Self {
        let status = HttpStatus::InternalServerError;
        let body = Some(body);

        Response { status, headers, body }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut response = format!("HTTP/1.1 {}\r\n", self.status.to_string());
        response.push_str(&self.headers.to_string());
        response.push_str("\r\n");
        let mut body_buffer:Vec<u8> = Vec::new();
        match &self.body {
            Some(body) => {
                body_buffer = body.to_buffer();
            },
            None => ()
        }
        let mut buffer = response.into_bytes();
        buffer.extend(body_buffer);
        buffer
    }
}





