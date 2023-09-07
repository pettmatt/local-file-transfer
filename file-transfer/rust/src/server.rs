use std::env;
use dotenv::dotenv;
use actix_web::{web, App, HttpServer};

use request_services::{frontpage, ping, get_devices, upload_file, download_file, get_local_files, remove_local_file};
// use process_http_request::handle_request;
pub mod custom_file;

mod custom_thread;
mod request_services;

#[actix_web::main]
pub async fn setup_server() -> std::io::Result<()> {
    dotenv().ok();

    let port: String = env::var("HOST_PORT").expect("Host port not defined");
    let host_address: String = env::var("HOST_ADDRESS").expect("Host address not defined");
    let workers: String = env::var("THREAD_POOL_COUNT").expect("Workers count not defined");

    println!("Server is running on http://{host_address}:{port}");

    HttpServer::new(|| {
        App::new()
            .service(ping)
            .service(frontpage)
            .service(get_devices)
            .service(get_local_files)
            .service(remove_local_file)
            .service(download_file)
            .route("/send-file", web::post().to(upload_file))
    })
    .workers(4)
    .bind((String::from(host_address), 7878))?
    .run()
    .await

}