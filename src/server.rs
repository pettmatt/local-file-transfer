use std::{fs, env};
use std::io::{self, Write};
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use dotenv::dotenv;
use serde_json::{Value};

use custom_thread::ThreadPool;
mod custom_thread;
use custom_file::FileObject;
mod custom_file;

// Handling backend of the user device
pub fn setup_server() {
    dotenv().ok();

    let port: String = env::var("HOST_PORT").expect("Host port not defined");
    let host_address: String = env::var("HOST_ADDRESS").expect("Host address not defined");
    let thread_pool_count: String = env::var("THREAD_POOL_COUNT").expect("Thread pool count not defined");

    let address = format!("{}:{}", host_address, port);
    let listener = TcpListener::bind(address).unwrap();
    let pool = ThreadPool::new(thread_pool_count.parse::<usize>().unwrap()); // Creating limited amount of threads. Performance can be increased by increasing the thread pool count.

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        })
    }
}

// Handling connections of the user (frontend activities) and connections of devices in the network
fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut request_line = String::new(); // &buf_reader.lines().next().unwrap().unwrap();
    buf_reader.read_line(&mut request_line).expect("Couldn't read request line");

    let request_line: &str = request_line.trim();
    println!("HANDLING");

    // Differenciate user requests from TCP connections of the application
    let response: String = match &request_line[..] {
        request if request.contains("HTTP/") => {
            println!("Handling HTTP");

            // Read the headers
            let mut headers = Vec::new();
            loop {
                let mut line = String::new();
                buf_reader.read_line(&mut line).expect("Couldn't read header line");
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
            buf_reader.read_exact(&mut body).expect("Couldn't read body");

            let body_str = String::from_utf8_lossy(&body).into_owned();

            let http_response = handle_http_request(request, body_str);
            format!("{}", http_response)
        }
        _ => {
            let received_file = FileObject::parse(&request_line);
            println!("Handling file {:?}", received_file);
            // fs::write works well only with text files.
            match fs::write(format!("file_tests/{}", received_file.name), received_file.content) {
                Ok(_response) => println!("File created successfully:"),
                Err(error) => println!("Creating file failed:\n{}", error)
            }

            let message = "Received a file";
            println!("Message: {:?}", message);
            format!("{:?}", message)
        }
    };

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_http_request(request_line: &str, body: String) -> String {
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

// Handling communication between host and other devices.
// Currently limited to sending files.
fn handle_sending_file(file_details: String) -> (&'static str, &'static str) {
    let file_json: Value = serde_json::from_str(&file_details).expect("Couldn't convert string to JSON");

    // let path = file_json["path"].as_str().expect("Path not found");
    let name = file_json["name"].as_str().expect("Name not found");
    let extension = file_json["extension"].as_str().expect("Extension not found");
    let file_name = format!("{}.{}", name, extension);
    println!("read file named {}", file_name);

    let file_string = read_file_utf16(&file_name).expect("Couldn't read file");
    let file_object = FileObject::new(String::from(file_name), file_string);
    println!("file contents {:?}", file_object);

    let response: Result<(), io::Error> = send_file_to_address("127.0.0.1:7878", file_object);
    let (mut status, mut message) = ("HTTP/1.1 200 OK", "Data was successfully delivered.");

    // Successful response when file has been sent successfully
    match response {
        Ok(res) => {
            println!("Data was successfully delivered.\r{:?}", res);
        }
        Err(error) => {
            println!("Error occured:\n{}", error);
            status = "HTTP/1.1 500 Internal Server Error";
            message = "Couldn't send the file";
        }
    }

    (status, message)
}

fn read_file_utf16(filename: &str) -> Result<String, io::Error> {
    let mut file = fs::File::open(filename).expect("Couldn't open the file");
    let metadata = file.metadata().expect("Couldn't read metadata of the file");
    let file_size = metadata.len() as usize;

    let mut contents = vec![0u8; file_size];
    file.read_exact(&mut contents).expect("Couldn't read the file");

    let file_str = String::from_utf8_lossy(&contents).into_owned();

    // Convert the bytes to a UTF-16 encoded string
    let utf16_string = format_into_utf16(file_str).expect("Couldn't convert file to UTF-16");
    Ok(utf16_string)
}

fn format_into_utf16(file_content: String) -> Result<String, io::Error> {
    // Filter map is used to filter out invalid unicodes which result in None from char::from_u32
    let utf16_content: String = file_content
        .encode_utf16()
        .filter_map(|c| char::from_u32(c as u32))
        .collect();

    Ok(utf16_content)
}

fn send_file_to_address(target_address: &'static str, file: FileObject) -> io::Result<()> {
    let mut stream = TcpStream::connect(target_address).expect("Couldn't create TCP connection");

    let serialized_data = file.serialize();
    let data = serialized_data.as_bytes();

    println!("sending this: {}", serialized_data);

    stream.write_all(data).expect("Couldn't buffer or validate the data");
    stream.flush().expect("Couldn't finish sending the data");

    Ok(())
}