use std::io::prelude::*;
use std::net::TcpStream;

pub struct Response {
    pub headers: String,
    pub body: String,
}

pub fn send_request_and_recv(stream: &mut TcpStream, host: &str) -> String {
    let request = format!("GET / HTTP/1.0\r\nAccept: */*\r\n Host: {}\r\n\r\n", host);

    stream.write(request.as_bytes()).unwrap();

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
        } if part == true {
            body.push_str(format!("{}\n", line).as_str());
        }
    }
    Response {
        headers: headers,
        body: body,
    }
}
