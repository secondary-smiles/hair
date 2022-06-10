mod args_lib;
mod cli_lib;
mod fn_lib;
mod parse;
mod struct_lib;
mod tcp_lib;

use fn_lib::{error, fenv_var};
use parse::parse_args;
use struct_lib::{Request, Url};
use tcp_lib::{connect_stream, send_request_and_recv};

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
                    port: None,
                },
            };
            error(&e.to_string(), 1);
        }
    }

    let mut stream = connect_stream(&request.url);

    let response = send_request_and_recv(&mut stream, &request);

    let print_verbose = fenv_var("HAIR_PRINT_VERBOSE");
    let print_headers = fenv_var("HAIR_PRINT_HEADERS");
    let print_body = fenv_var("HAIR_PRINT_BODY");

    if print_headers == '1'.to_string() && print_verbose != '1'.to_string() {
        println!("{}", response.headers);
    }
    if print_body == '1'.to_string() && print_verbose != '1'.to_string() {
        println!("{}", response.body);
    }

    if print_body != '1'.to_string() && print_headers != '1'.to_string()
        || print_verbose == '1'.to_string()
    {
        println!("{}\n\n{}", response.headers, response.body);
    }

    match stream.flush() {
        Ok(_) => (),
        Err(e) => error(&e.to_string(), 1),
    }
}

fn init_env() {
    let print_verbose = 0;
    let print_body = 0;
    let print_headers = 0;
    let use_old_protocol = 0;
    let parse_err_dump = 0;

    std::env::set_var("HAIR_PRINT_VERBOSE", print_verbose.to_string());
    std::env::set_var("HAIR_PRINT_BODY", print_body.to_string());
    std::env::set_var("HAIR_PRINT_HEADERS", print_headers.to_string());
    std::env::set_var("HAIR_USE_OLD_PROTOCOL", use_old_protocol.to_string());
    std::env::set_var("HAIR_PARSE_ERR_DUMP", parse_err_dump.to_string());
}
