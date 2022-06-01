use std::io::prelude::*;
use std::net::TcpStream;

use super::cli_lib::VERSION;
use super::struct_lib::{Request, Response};

pub fn connect_stream(host: &str) -> TcpStream {
    let mut stream =
        TcpStream::connect(format!("{}:80", host)).expect("Could not connect to server");
    stream.flush().unwrap();
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

    
    let print_verbose = std::env::var("PRINT_VERBOSE")
    .unwrap()
    .parse::<i32>()
    .unwrap();
    
    if print_verbose == 1 {
        println!("{}", send_request);
    }

    stream.write(send_request.as_bytes()).unwrap();

    let mut data = Vec::new();
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = stream.read(&mut buffer).unwrap();
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
