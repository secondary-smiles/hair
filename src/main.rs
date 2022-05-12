mod lib;
use lib::{ send_request_and_recv, parse_request };
use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let host = "gnu.org";
    let mut stream =
        TcpStream::connect(format!("{}:80", host)).expect("Could not connect to server");

    let raw_data = send_request_and_recv(&mut stream, &host);
    let response = parse_request(raw_data);

    println!("{}", response.headers);

    stream.flush().unwrap();
}
