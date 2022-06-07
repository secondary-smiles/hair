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

    let send_request = format!(
        // Request: GET, Path-as-parsed, HTTP/1.1, Accept: */* Host: Host-as-parsed User-Agent: hair/0.1.1
        "{} {} HTTP/1.0\r\nAccept: */*\r\nHost: {}\r\nUser-Agent: {}\r\n\r\n",
        request.method.as_ref().unwrap_or(&"GET".to_string()),
        request.url.path,
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
    let mut buffer = [0; 1024];
    let mut header_buffer = [0; 8192];

    match stream.read(&mut header_buffer) {
        Ok(b) => b,
        Err(e) => {
            error(&e.to_string(), 1);
            0
        },
    };

    check_chunked_encoding(&header_buffer);
    let first_response = true;

    loop {
        let bytes_read = match stream.read(&mut buffer) {
            Ok(b) => b,
            Err(e) => {
                error(&e.to_string(), 1);
                0
            },
        };
        if first_response {
            header_buffer.copy_from_slice(&buffer[..bytes_read]);
        }
        if bytes_read == 0 {
            break;
        }

        data_buffer.extend_from_slice(&buffer[..bytes_read]);
        //println!("{}", String::from_utf8_lossy(&data));
    }

    //let data = String::from_utf8_lossy(&data);

    return add_buffers_to_response(&header_buffer, &data_buffer);

    //return parse_request(data.to_string());
}

fn parse_request(request: String) -> Response {
    let parts = request.split("\r\n\r\n").collect::<Vec<&str>>();
    if parts.len() < 2 {
        error(&"Invalid server response, failed to parse for headers".to_string(), 1);
    }
    Response {
        headers: parts[0].to_string(),
        body: parts[1].to_string(),
    }
}

fn check_chunked_encoding(buffer: &[u8]) {
    let inital_chunk = String::from_utf8_lossy(buffer);
    let _inital_chunk_response = parse_request(inital_chunk.to_string());
    //println!("{}", inital_chunk_response.headers);
}

fn add_buffers_to_response(header_buffer: &[u8], data_buffer: &[u8]) -> Response {
    Response {
        headers: String::from_utf8_lossy(header_buffer).to_string(),
        body: String::from_utf8_lossy(data_buffer).to_string(),
    }
}
