use std::io::prelude::*;
use std::net::TcpStream;

use super::cli_lib::VERSION;
use super::struct_lib::{Request, Response};
use super::fn_lib::{error};

pub fn connect_stream(host: &str) -> TcpStream {
    let mut stream: TcpStream;
    match TcpStream::connect(format!("{}:80", host)) {
        Ok(s) => stream = s,
        Err(e) =>{ 
            error(&e.to_string(), 1);
            stream = TcpStream::connect(format!("{}:80", host)).unwrap();
        },
    }
    match stream.flush() {
        Ok(_) => (),
        Err(e) => error(&e.to_string(), 1),
    }
    stream
}

pub fn send_request_and_recv(stream: &mut TcpStream, request: &Request) -> String {
    let user_agent: String = format!("hair/{}", VERSION);

    let send_request = format!(
        // Request: GET, Path-as-parsed, HTTP/1.1, Accept: */* Host: Host-as-parsed User-Agent: hair/0.1.1
        "{} {} HTTP/1.0\r\nAccept: */*\r\nHost: {}\r\nUser-Agent: {}\r\n\r\n",
        request.method.as_ref().unwrap_or(&"GET".to_string()),
        request.url.path,
        request.url.host,
        user_agent
    );

    let print_verbose = match std::env::var("HAIR_PRINT_VERBOSE") {
        Ok(v) => v,
        Err(_) => {
            let msg = "Could not find ENV variable".to_string();
            error(&msg, 1);
            "0".to_string()
        },
    };
    
    if print_verbose == '1'.to_string() {
        println!("{}", send_request);
    }

    match stream.write(send_request.as_bytes()) {
        Ok(_) => (),
        Err(e) => error(&e.to_string(), 1),
    }

    let mut data = Vec::new();
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = match stream.read(&mut buffer) {
            Ok(b) => b,
            Err(e) => {
                error(&e.to_string(), 1);
                0
            },
        };
        if bytes_read == 0 {
            break;
        }

        data.extend_from_slice(&buffer[..bytes_read]);
    }

    let data = String::from_utf8_lossy(&data);

    return data.to_string();
}

pub fn parse_request(request: String) -> Response {
    let lines: Vec<&str> = request.split("\r\n").collect();
    let mut headers = String::new();
    let mut body = String::new();
    let mut part = false;
    for line in lines {
        if line != "" && part == false {
            headers.push_str(format!("{}\n", line).as_str());
        } else if line == "" && part == false {
            part = true;
        }
        if part == true {
            body.push_str(format!("{}\n", line).as_str());
        }
    }
    Response {
        headers: headers.to_string(),
        body: body.to_string(),
    }
}
