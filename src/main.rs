mod lib;
use lib::{ send_request_and_recv, parse_request, Request };
use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    
    let request = Request {
        method: "GET".to_string(),
        path: "/".to_string(),
        host: "linux.org".to_string(),
    };
    
    let mut stream =
        TcpStream::connect(format!("{}:80", request.host)).expect("Could not connect to server");

    let raw_data = send_request_and_recv(&mut stream, &request);
    let response = parse_request(raw_data);

    println!("{}", response.body);

    stream.flush().unwrap();
}
