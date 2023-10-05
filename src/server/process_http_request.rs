use std::io::{prelude::*, BufReader};
use std::net::TcpStream;
use std::fs;

mod process_file;
use process_file::handle_sending_file;

pub use super::custom_file::FileObject;

pub fn fetch_details_from_request(mut reader: BufReader<&mut TcpStream>) -> (Vec<String>, usize, String) {
    // Read the headers
    let mut headers = Vec::new();
    loop {
        let mut line = String::new();
        reader.read_line(&mut line).expect("Couldn't read header line");
        if line == "\r\n" {
            break;
        }
        headers.push(line);
    }

    // Extract content length from headers
    let content_length: usize = headers.iter()
        .find(|header| header.to_lowercase().starts_with("content-length"))
        .and_then(|header: &String| header.split(':').nth(1))
        .and_then(|length| length.trim().parse().ok())
        .unwrap_or(0);

    // Get body of the request. We need to define the length of the content so the server won't be stuck
    let mut body = vec![0u8; content_length];
    reader.read_exact(&mut body).expect("Couldn't read body");
    let body_string = String::from_utf8_lossy(&body).into_owned();

    (headers, content_length, body_string)
}

pub fn handle_http_request(request_line: &str, body: String) -> String {
    let request = String::from(request_line);

    let (status_line, filename) = match &request[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /ping HTTP/1.1" => ("HTTP/1.1 200 OK", "{\"message\": \"pong\"}"),
        "POST /send HTTP/1.1" => handle_sending_file(body),
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let mut contents = filename.to_string();

    match fs::read_to_string(filename) {
        Ok(string) => {
            contents = string;
        }
        Err(_error) => {
            // If the file couldn't be found, the value is probably JSON object and should be kept as is.
            // println!("The file doesn't exist: {}", _error);
        }
    }

    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    return response
}