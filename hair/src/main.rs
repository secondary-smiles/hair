mod tcp_lib;
mod parse;
mod struct_lib;
mod args_lib;
mod cli_lib;

use parse::{parse_args};
use struct_lib::{Request};
use tcp_lib::{parse_request, send_request_and_recv, connect_stream };


use std::io::prelude::*;
use std::env::args;

fn main() {
    init_env();

    let request: Request = parse_args(args().collect()).unwrap();

    let mut stream = connect_stream(&request.url.host);

    let raw_data = send_request_and_recv(&mut stream, &request);
    let response = parse_request(raw_data);

    println!("{}{}", response.headers, response.body);

    stream.flush().unwrap();
    // We don't need this yet because with http/1.0 the connection is closed automatically
    //stream.shutdown(Shutdown::Both).expect("Shutdown call failed");
}

fn init_env() {
    let print_verbose = 0;
    std::env::set_var("PRINT_VERBOSE", print_verbose.to_string());
}
