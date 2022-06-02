mod args_lib;
mod cli_lib;
mod fn_lib;
mod parse;
mod struct_lib;
mod tcp_lib;

use fn_lib::{error};
use parse::{parse_args};
use struct_lib::{Request, Url};
use tcp_lib::{connect_stream, parse_request, send_request_and_recv};

use std::env::args;
use std::io::prelude::*;

fn main() {
    init_env();

    let request: Request;
    match parse_args(args().collect()) {
        Ok(r) => request = r,
        Err(e) => {
            request = Request {
                method: None,
                url: Url {
                    host: "".to_string(),
                    path: "".to_string(),
                },
            };
            error(e, 1);
        }
    }

    let mut stream = connect_stream(&request.url.host);

    let raw_data = send_request_and_recv(&mut stream, &request);
    let response = parse_request(raw_data);

    println!("{}{}", response.headers, response.body);

    stream.flush().unwrap();
}

fn init_env() {
    let print_verbose = 0;
    std::env::set_var("HAIR_PRINT_VERBOSE", print_verbose.to_string());
}
