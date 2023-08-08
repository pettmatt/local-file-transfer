use std::{fs, env};
use std::io::{self, Write};
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use dotenv::dotenv;

use serde_json;

use custom_thread::ThreadPool;
mod custom_thread;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct FileObject {
    name: String,
    content: String
}

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
    let buf_reader = BufReader::new(&mut stream);
    let request_line = &buf_reader.lines().next().unwrap().unwrap();

    // Differenciate user requests from TCP connections of the application
    let response: String = match &request_line[..] {
        request if request.contains("HTTP/") => {
            let http_response = handle_http_request(request);
            format!("{}", http_response)
        }
        _ => {
            println!("REQUEST {}", request_line);
            let mut reader = BufReader::new(&mut stream);
            let received: Vec<u8> = reader.fill_buf().expect("failed").to_vec();

            reader.consume(received.len());

            let message = String::from_utf8(received)
                .map(|message| println!("{}", message))
                .map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Couldn't parse received string as utf8"
                    )
                });

            // let message = "Received a file";
            println!("Message: {:?}", message);
            format!("{:?}", message)
        }
    };

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_http_request(request_line: &str) -> String {
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /ping HTTP/1.1" => ("HTTP/1.1 200 OK", "{\"message\": \"pong\"}"),
        "POST /send HTTP/1.1" => handle_sending_file(),
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
fn handle_sending_file() -> (&'static str, &'static str) {
    let filename = "text_test.txt";
    let file_contents = &fs::read(filename).expect("Couldn't read the file");
    let file_string: std::borrow::Cow<'_, str> = String::from_utf8_lossy(file_contents);
    let file_object = FileObject {
        name: String::from(filename),
        content: String::from(file_string)
    };

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

fn send_file_to_address(address: &'static str, file: FileObject) -> io::Result<()> {
    let mut stream = TcpStream::connect(address).expect("Couldn't create TCP connection");

    let serialized_data = serde_json::to_string(file).expect("Serializing failed");
    let data = file.content.as_bytes();

    stream.write_all(data).expect("Couldn't buffer or validate the data");
    stream.flush().expect("Couldn't finish sending the data");

    Ok(())
}