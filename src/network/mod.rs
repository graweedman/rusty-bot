pub mod headers;
pub mod request;
pub mod response;
pub mod router;

use headers::Headers;
use request::{method::Method, Request};
use response::{body::Body, Response};
use router::{controllers::main_controller::MainController, Router};

use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

use self::router::controllers::{self, main_controller, Controllers};

pub struct Networking {
    pub host: String,
    pub port: String,
    listener: TcpListener,
    router: Router,
    controllers: Controllers,
}

impl Networking {
    pub fn get_endpoint(host: &str, port: &str) -> String {
        host.to_owned() + ":" + port
    }

    pub fn connect(host: &str, port: &str) -> Result<Self, &'static str> {
        let listener = TcpListener::bind(Self::get_endpoint(host, port)).expect("Invalid Endpoint");
        let router = Router::new();
        let host: String = host.to_string();
        let port: String = port.to_string();
        let controllers = Controllers::new();

        Ok(Networking {
            host,
            port,
            listener,
            router,
            controllers,
        })
    }

    pub fn start_server(&self) {
        println!("Web server is listening at port {}", self.port);
        for stream in self.listener.incoming() {
            let _stream = stream.expect("Failed to establish connection");
            // Call function to process any incomming connections
            self.handle_connection(_stream);
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).expect("Failed to read stream");
        let request_str = std::str::from_utf8(&buffer).expect("Invalid UTF-8");

        let request: Request<String> = Request::parse(request_str).expect("ParseFailed");

        println!("Request received: {}", request.get_path().to_string());

        let response = self.router.route_request(request);
        let res_buffer = response.serialize();

        match stream.write_all(&res_buffer) {
            Ok(_) => println!("Response sent"),
            Err(err) => println!("Failed sending response: {:?}", err),
        };

        stream.flush().expect("Failed to flush stream");
    }
}
