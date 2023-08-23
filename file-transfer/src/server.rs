use std::env;
use dotenv::dotenv;
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;

use process_http_request::handle_request;
mod custom_thread;
pub mod custom_file;
mod process_http_request;

#[tokio::main]
pub async fn setup_server() {
    dotenv().ok();

    let port: String = env::var("HOST_PORT").expect("Host port not defined");
    let host_address: String = env::var("HOST_ADDRESS").expect("Host address not defined");
    // let thread_pool_count: String = env::var("THREAD_POOL_COUNT").expect("Thread pool count not defined");
    let server_address = format!("{}:{}", host_address, port);

    let service = make_service_fn(|_connection| {
        async {
            Ok::<_, hyper::Error>(service_fn(handle_request)) 
        }
    });

    let address = server_address.parse().unwrap();
    let server = Server::bind(&address).serve(service);

    println!("Server running at http://{}", address);

    if let Err(error) = server.await {
        eprintln!("server error: {}", error);
    }
}