use std::{fs, env};
use std::io::{self, Write};
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use dotenv::dotenv;

use custom_thread::ThreadPool;
mod custom_thread;

// Handling backend of the user device
pub fn setup_server() {
    dotenv().ok();

    let port: String = env::var("HOST_PORT").expect("Host port not defined");
    let host_address: String = env::var("HOST_ADDRESS").expect("Host address not defined");
    let thread_pool_count: String = env::var("THREAD_POOL_COUNT").expect("Thread pool count not defined");

    let address = format!("{}:{}", host_address, port);
    let listener = TcpListener::bind(address).unwrap();
    let pool = ThreadPool::new(thread_pool_count.parse::<usize>().unwrap()); // limited amount of threads.

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        })
    }
}

// Handling connections of the user (frontend activities) and possible devices in the network.
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /ping HTTP/1.1" => ("HTTP/1.1 200 OK", "{\"message\": \"pong\"}"),
        "GET /process_file HTTP/1.1" => ("", ""),
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

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

// Handling communication between host and other devices.
// Currently limited to sending files.
fn handle_sending_file() -> (&'static str, &'static str) {
    let response: Result<(), io::Error> = send_file_to_address("127.0.0.1:7878");

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

fn send_file_to_address(address: &'static str) -> io::Result<()> {
    let mut stream = TcpStream::connect(address).expect("Couldn't create TCP connection");

    let data = b"Hello";
    stream.write_all(data).expect("Couldn't buffer or validate the data");
    stream.flush().expect("Couldn't finish sending the data");

    Ok(())
}