use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::fs;

pub mod request;
pub mod response;
pub mod router;
pub mod headers;

use request::Request;
pub struct Networking {
    pub host: String,
    pub port: String,
    listener: TcpListener,
}

impl Networking {

    pub fn get_endpoint(host:&str, port: &str) -> String {
        host.to_owned() + ":" + port
    }

    pub fn connect(host: &str, port: &str) -> Result<Self, &'static str> {
        let listener = TcpListener::bind(Self::get_endpoint(host, port)).expect("Invalid Endpoint");
        let host: String = host.to_string();
        let port: String = port.to_string();
        Ok(Networking { host, port, listener })
    }

    pub fn start_server(&self) {
        println!("Web server is listening at port {}", self.port);
        for stream in self.listener.incoming() {
            let _stream = stream.unwrap();
            // Call function to process any incomming connections
            self.handle_connection(_stream);
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let request_str = std::str::from_utf8(&buffer).unwrap();

        let request: Request<String> = Request::parse(request_str).expect("ParseFailed");

        // println!("Request received: {}", request.path);

        let response_contents = fs::read_to_string("src/index.html").unwrap();
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            response_contents.len(),
            response_contents
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

