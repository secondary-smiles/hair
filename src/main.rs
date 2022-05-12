mod lib;

use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let host = "gnu.org";
    let mut stream =
        TcpStream::connect(format!("{}:80", host)).expect("Could not connect to server");

    let raw_data = lib::send_request_and_recv(&mut stream, &host);
    let split_data = lib::parse_request(raw_data);

    println!("{}", split_data[1]);

    stream.flush().unwrap();
}
