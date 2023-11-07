
/* Simple HTTP Server */
/* Author : Ramesh Vyas */
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};


mod networking;

use networking::Networking;


fn main() {
    /* Creating a Local TcpListener at Port 8477 */
    const HOST : &str ="127.0.0.1";
    const PORT : &str ="8477";
    /* Concating Host address and Port to Create Final Endpoint */
    let network: Networking = Networking::connect(HOST, PORT).expect("Connection Error");

    network.start_server();





    /* Conneting to any incoming connections */

}




