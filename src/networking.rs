use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::fs;

pub mod request;
pub mod response;
pub mod router;
pub mod headers;
pub mod body;

use request::Request;
use router::Router;
use response::Response;
use headers::Headers;

use request::method::Method;

use self::body::Body;
pub struct Networking {
    pub host: String,
    pub port: String,
    listener: TcpListener,
    router: Router
}

impl Networking {

    pub fn get_endpoint(host:&str, port: &str) -> String {
        host.to_owned() + ":" + port
    }

    pub fn connect(host: &str, port: &str) -> Result<Self, &'static str> {
        let listener = TcpListener::bind(Self::get_endpoint(host, port)).expect("Invalid Endpoint");
        let mut router = Router::new();
        let host: String = host.to_string();
        let port: String = port.to_string();

        router.add_route(Method::GET, "/", |request| {
            let mut headers = Headers::new();
            headers.add_header("Content-Type", "text/html");
            let response = match fs::read_to_string("static/index.html") {
                Ok(file) => Response::ok(Body::new(file), headers),
                Err(err) => Response::internal_server_error(Body::new(format!("500: File Read error: {:?}", err)), headers)
            };
            response
        });
        
        Ok(Networking { host, port, listener, router })
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

        println!("Request received: {}", request.get_path());
        
        let response = self.router.route_request(request);
        let res_buffer = response.serialize();
        
        match stream.write_all(&res_buffer) {
            Ok(_) => println!("Response sent"),
            Err(err) => println!("Failed sending response: {:?}", err)
        };
        
        stream.flush().unwrap();
    }
}

