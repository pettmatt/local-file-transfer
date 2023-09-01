use std::net::{SocketAddrV4, IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::path::Path;
use actix_web::{get, post, web, error, HttpRequest, HttpResponse, Responder, http::header::CONTENT_LENGTH};
use actix_multipart::Multipart;
use serde_json::Value;

use request_processes::get_local_devices;
// use process_file::handle_sending_file;

mod process_file;
mod request_processes;
pub mod custom_ip_utils;

#[get("/")]
pub async fn frontpage() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/ping")]
pub async fn ping(req_body: String) -> impl Responder {
    let devices = get_local_devices().unwrap();

    let response: Value = json!({
        "message": "Pong"
    });

    HttpResponse::Ok()
        .content_type("application/json")
        .json(response)
}

#[get("/devices")]
pub async fn get_devices() -> impl Responder {
    let devices = get_local_devices().unwrap();

    let response: Value = json!({
        "devices": devices
    });

    HttpResponse::Ok()
        .content_type("application/json")
        .body(response)
}

pub async fn receive_file(mut payload: Multipart, request: HttpRequest) -> impl Responder {

    // 1) Receive file.
    // 2) Save the file locally in a folder.

    let content_length: usize = match request.headers().get(CONTENT_LENGTH) {
        Some(header_value) => header_value.to_str().unwrap_or("0").parse().unwrap(),
        None => 0
    };

    let file_count: usize = 0;
    let mut current_count: usize = 0;
    let directory: &str = "./upload/";

    // loop {
    //     if current_count == file_count {
    //         break;
    //     }

    //     if let Ok(Some(mut field)) = payload.try_next().await {
    //         let filetype: Option<&Mime> = field.content_type();
    //         if filetype.is_none() { 
    //             continue; 
    //         }

    //         let destination: String = format!(
    //             "{}{}-{}",
    //             directory,
    //             field.content_disposition().get_filename().unwrap()
    //         );

    //         let mut saved_file: fs::File = fs::File::create(&destination).unwrap();
    //         while let Ok(Some(chunk)) = field.try_next().await {
    //             let _ = saved_file.write_all(&chunk).await.unwrap();
    //         }
    //     } else {
    //         break;
    //     }

    //     current_count += 1;
    // }

    // let (status, message) = handle_sending_file(&body);
    // let response_json = web::Json({ message: "Still static", body: body });

    HttpResponse::Ok()
        .content_type("application/json")
        .body("This is in testing phase")
}
