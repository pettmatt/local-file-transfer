use std::env;
use dotenv::dotenv;
use actix_web::{web, App, HttpServer};
use actix_cors::Cors;

mod request_services;
use request_services::{frontpage, ping, upload_file, download_file, get_local_files, remove_local_file};

#[actix_web::main]
pub async fn setup_server() -> std::io::Result<()> {
    dotenv().ok();

    let port: String = env::var("HOST_PORT").expect("Host port not defined");
    let host_address: String = env::var("HOST_ADDRESS").expect("Host address not defined");
    let workers: String = env::var("THREAD_POOL_COUNT").expect("Workers count not defined");

    println!("Server is running on http://{host_address}:{port}");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin() // Feel free to replace with specific url
            // .allowed_origin("http://127.0.0.1:5173") // Example how to specify allowed origin url
            .allowed_methods(vec!["GET", "POST", "DELETE"])
            .supports_credentials();

        App::new()
            .wrap(cors)
            .service(ping)
            .service(frontpage)
            .service(get_local_files)
            .service(remove_local_file)
            .service(download_file)
            .route("/send", web::post().to(upload_file))
    })
    .workers(4)
    .bind((String::from(host_address), 7878))?
    .run()
    .await
}