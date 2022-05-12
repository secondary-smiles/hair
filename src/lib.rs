use std::io::prelude::*;
use std::net::TcpStream;

pub fn send_request_and_recv(stream: &mut TcpStream, host: &str) -> String {
    let request = format!(
        "GET / HTTP/1.0\r\nHost: www.{}\r\nAccept-Language: en-us\r\n\r\n",
        host
    );

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

pub fn parse_request(request: String) -> Vec<String> {
    let lines: Vec<&str> = request.split("\r\n").collect();
    let mut response = String::new();
    let mut contents = String::new();
    let mut part = false;
    for line in lines {
        if line != "" && part == false {
            response.push_str(format!("{}\n", line).as_str());
        } else if line == "" && part == false {
            part = true;
        } else if line != "" && part == true {
            contents.push_str(format!("{}\n", line).as_str());
        }
    }
    vec![response, contents]
}
