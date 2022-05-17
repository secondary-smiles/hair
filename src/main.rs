// Other files
mod tcp_lib;
mod cli_lib;
mod struct_lib;

// Local dependencies
use struct_lib::{ Request };
use tcp_lib::{parse_request, send_request_and_recv };
use cli_lib:: { parse_args };

// Dependencies
use std::io::prelude::*;
use std::net::{ TcpStream, Shutdown };
use std::env::args;

fn main() {
    let request: Request = parse_args(&mut args().collect()).unwrap();

    let mut stream =
        TcpStream::connect(format!("{}:80", request.url.host)).expect("Could not connect to server");

    let raw_data = send_request_and_recv(&mut stream, &request);
    let response = parse_request(raw_data);

    println!("{}{}", response.headers, response.body);

    stream.flush().unwrap();
    stream.shutdown(Shutdown::Both).expect("Shutdown call failed");
}
