use std::{fs, env};
use std::io::Write;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use dotenv::dotenv;

mod custom_thread;
use custom_thread::ThreadPool;
pub mod custom_file;
use custom_file::FileObject;
mod process_http_request;
use process_http_request::{fetch_details_from_request, handle_http_request};

// mod server {
//     pub mod custom_file;
//     pub mod process_http_request {
//         pub mod process_file;
//     }
// }

// Handling backend of the user device
pub fn setup_server() {
    dotenv().ok();

    let port: String = env::var("HOST_PORT").expect("Host port not defined");
    let host_address: String = env::var("HOST_ADDRESS").expect("Host address not defined");
    let thread_pool_count: String = env::var("THREAD_POOL_COUNT").expect("Thread pool count not defined");
    let server_address = format!("{}:{}", host_address, port);

    match TcpListener::bind(server_address) {
        Ok(listener) => {
            println!("Server is running on {}", format!("{}:{}", host_address, port));

            let pool = ThreadPool::new(thread_pool_count.parse::<usize>().unwrap()); // Creating limited amount of threads. Performance can be increased by increasing the thread pool count.

            for stream in listener.incoming() {
                let stream = stream.unwrap();
        
                pool.execute(|| {
                    handle_connection(stream);
                })
            }
        },
        Err(error) => println!("Failed to bind port to {host_address} : {error}")
    };
}

// Handling connections of the user (frontend activities) and connections of devices in the network
fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut request_line = String::new();
    buf_reader.read_line(&mut request_line).expect("Couldn't read request line");

    let request_line: &str = request_line.trim();

    // Differenciate user requests from TCP connections of the application
    let response: String = match &request_line[..] {
        request if request.contains("HTTP/") => {
            println!("Handling HTTP");

            let (_headers, _content_length, body) = fetch_details_from_request(buf_reader);
            let http_response = handle_http_request(request, body);

            format!("{}", http_response)
        }
        _ => {
            let received_file = FileObject::parse(&request_line);
            println!("Handling file {:?}", received_file);

            // This approach works well with text files, but it needs some tweaking
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