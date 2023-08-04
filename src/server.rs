use std::{fs, env, thread};
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use dotenv::dotenv;

use custom_thread::ThreadPool;
mod custom_thread;

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

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /ping HTTP/1.1" => ("HTTP/1.1 200 OK", ""),
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let mut contents = String::new();

    match fs::read_to_string(filename) {
        Ok(string) => {
            println!("PATH {:?}", string);
            contents = string;
        }
        Err(error) => {
            println!("The file doesn't exist: {}", error);
        }
    }

    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}