use std::io::prelude::*;
use std::net::TcpStream;

use super::cli_lib::VERSION;
use super::struct_lib::{Request, Response, Url};
use super::fn_lib::{error, fenv_var};

pub fn connect_stream(url: &Url) -> TcpStream {
    let mut stream: TcpStream;
    match TcpStream::connect(format!("{}:{}", url.host, url.port.clone().unwrap_or("80".to_string()))) {
        Ok(s) => stream = s,
        Err(e) =>{ 
            error(&e.to_string(), 1);
            stream = TcpStream::connect(format!("{}:{}", url.host, url.port.clone().unwrap_or("80".to_string()))).unwrap();
        },
    }
    match stream.flush() {
        Ok(_) => (),
        Err(e) => error(&e.to_string(), 1),
    }
    stream
}

pub fn send_request_and_recv(stream: &mut TcpStream, request: &Request) -> Response {
    let user_agent: String = format!("hair/{}", VERSION);
    let mut http_protocol_version = "1.1";
    if fenv_var("HAIR_USE_OLD_PROTOCOL") == "1".to_string() {
        http_protocol_version = "1.0";
    }

    let send_request = format!(
        // Request: {GET} {Path-as-parsed} HTTP/{http_protocol_version} Accept: */* Host: {Host-as-parsed} User-Agent: hair/{0.1.1}
        "{} {} HTTP/{}\r\nAccept: */*\r\nHost: {}\r\nUser-Agent: {}\r\n\r\n",
        request.method.as_ref().unwrap_or(&"GET".to_string()),
        request.url.path,
        http_protocol_version,
        request.url.host,
        user_agent
    );

    let print_verbose = fenv_var("HAIR_PRINT_VERBOSE");
    
    if print_verbose == '1'.to_string() {
        println!("{}", send_request);
    }

    match stream.write(send_request.as_bytes()) {
        Ok(_) => (),
        Err(e) => error(&e.to_string(), 1),
    }

    let mut data_buffer = Vec::new();
    let mut buffer = [0; 8192];
    let mut first_response = true;
    let mut response_type = None;

    loop {
        let bytes_read = match stream.read(&mut buffer) {
            Ok(b) => b,
            Err(e) => {
                error(&e.to_string(), 1);
                0
            },
        };
        if first_response == true {
            response_type = response_end_indicator(&buffer);
            if response_type == Some(false) {
                parse_content_length(&buffer);
            }
            first_response = false;
        }
        match response_type {
            None => {
                println!("Checking end for unknown response terminator");
                if bytes_read == 0 {
                    break;
                }
            },
            Some(true) => {
                println!("Checking end for chunked encoding");
                if bytes_read == 0 {
                    break;
                }
            },
            Some(false) => {
                println!("Checking end for content-length encoding");
                if bytes_read == 0 {
                    break;
                }
            },
        }

        data_buffer.extend_from_slice(&buffer[..bytes_read]);
    }

    return parse_request(&buffer);
}

fn parse_request(request: &[u8]) -> Response {
    let data_string = String::from_utf8_lossy(&request);
    let parts = data_string.split("\r\n\r\n").collect::<Vec<&str>>();
    if parts.len() < 2 {
        error(&"Invalid server response, failed to parse for headers".to_string(), 1);
    }
    Response {
        headers: parts[0].to_string(),
        body: parts[1].to_string(),
    }
}

fn response_end_indicator(buffer: &[u8]) -> Option<bool> {
    // Chunked Transfer Encoding: True; Content-Length: False; Unknown: None;
    let inital_chunk_response = parse_request(buffer);
    if inital_chunk_response.headers.contains("Transfer-Encoding: chunked") {
        return Some(true);
    }
    if inital_chunk_response.headers.contains("Content-Length:") {
        return Some(false);
    }

    return None;
}

fn parse_content_length(buffer: &[u8]) -> i32 {
    let data = parse_request(buffer).headers;
    let content_length_vec = data.split("Content-Length: ").collect::<Vec<&str>>();
    if content_length_vec.len() < 2 {
        error(&"Invalid server response, failed to parse for content-length".to_string(), 1);
    }
    let content_length_str = content_length_vec[1].split("\r\n").collect::<Vec<&str>>()[0];
    let content_length_int = match content_length_str.parse::<i32>() {
        Ok(i) => i,
        Err(e) => {
            error(&format!("Invalid server response, failed to parse for content-length: {}: {:?}", e, content_length_str), 1);
            0
        },
    };
    println!("{:?}", content_length_int);
    0
}
