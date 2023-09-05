use actix_web::{get, Error, HttpResponse, HttpRequest, Responder, http::header::CONTENT_LENGTH};
use actix_multipart::Multipart;
use futures_util::StreamExt;
use tokio::io::AsyncWriteExt;

// use request_processes::get_local_devices;
// use process_file::handle_sending_file;

// mod process_file;
mod request_processes;
pub mod custom_ip_utils;

#[get("/")]
pub async fn frontpage() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/ping")]
pub async fn ping(_ping_address: String) -> impl Responder {
    let response = json!({
        "message": "Pong"
    });

    HttpResponse::Ok()
        .content_type("application/json")
        .json(response)
}

#[get("/devices")]
pub async fn get_devices() -> impl Responder {

    // 1) Check devices in the network by pinging to address of the server.
    //  1.1) If device responds the device has the same application.
    // 2) Return the retrieved device details.

    // let devices = get_local_devices().unwrap();

    // let response = json!({
    //     "devices": devices
    // });

    let response = String::from("Hard coded value");

    HttpResponse::Ok()
        .content_type("application/json")
        .body(response)
}

pub async fn receive_file(mut payload: Multipart, request: HttpRequest) -> Result<HttpResponse, Error> {

    // 1) Receive file.
    // 2) Save the file locally.
    //  2.1) Check if local directory exists.
    //  2.2) Create the directory if needed.
    //  2.3) Create file.

    let content_length: usize = match request.headers().get(CONTENT_LENGTH) {
        Some(header_value) => header_value.to_str().unwrap_or("0").parse().unwrap(),
        None => 0
    };

    let mut files_created = Vec::new();
    let mut file_count = 0;

    while let Some(item) = payload.next().await {
        file_count = file_count + 1;
        let mut field = item?;

        let filename = field.content_disposition()
            .get_filename()
            .unwrap_or("unknown");

        let read_result = tokio::fs::read_dir("./uploads")
            .await
            .map_err(|error| { error });

        match read_result {
            Err(err) => {
                // Directory was not found, which is identified as code "3"
                if err.raw_os_error() == Some(3) {
                    let _ = tokio::fs::create_dir("./uploads")
                        .await
                        .map_err(|error| {
                            eprintln!("Error creating directory: {:?}", error);
                        });
                }
            },
            _ => println!("Application has existing 'uploads' directory")
        }

        let filepath = format!("./uploads/{filename}");

        let mut saved_file = tokio::fs::File::create(filepath).await.unwrap();
        while let Some(chunk) = field.next().await {
            let chunk = chunk.unwrap();
            let _ = saved_file.write_all(&chunk).await.unwrap();
        }

        files_created.push(saved_file);
    }

    let message = format!("loaded {} out of {} files", files_created.len(), file_count);
    let response = json!({
        "status": "success",
        "message": message
    });

    Ok(
        HttpResponse::Ok()
            .content_type("application/json")
            .json(response)
    )
}
