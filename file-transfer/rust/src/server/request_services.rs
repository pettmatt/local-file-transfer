use actix_web::{web, get, delete, Error, HttpResponse, HttpRequest, Responder};
use actix_multipart::Multipart;
use actix_files::NamedFile;
use futures_util::StreamExt;
use tokio::io::AsyncWriteExt;
use mime::Mime;
use tokio::fs;
use std::path::PathBuf;

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

#[get("/local-files")]
pub async fn get_local_files() -> Result<HttpResponse, Error> {

    let mut files: Vec<String> = Vec::new();
    let entries = std::fs::read_dir("./uploads")?;

    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name(); // returns OsString
        let file_name_string = file_name.to_string_lossy();
        files.push(file_name_string.to_string());
    }

    let response = json!({
        "files": files
    });

    Ok(
        HttpResponse::Ok()
            .content_type("application/json")
            .json(response)
    )
}

#[derive(serde::Deserialize, Debug)]
struct QueryParams {
    file_name: String,
}

#[delete("/local-file")]
pub async fn remove_local_file(query_params: web::Query<QueryParams>) -> Result<HttpResponse, Error> {

    // 1) Get name of the file from request params.
    // 2) Search file with the same name and delete it.

    let file_name = &query_params.file_name.to_string();
    let found_file = check_if_file_exists(&file_name).unwrap();

    let mut response = json!({
        "message": format!("File '{}' deleted '{}'", file_name, found_file)
    });

    if found_file == "true" {
        let destination = format!("./uploads/{}", &file_name);
        let file_deletion = std::fs::remove_file(destination);

        match file_deletion {
            Ok(()) => {},
            Err(error) => {
                response = json!({
                    "message": format!("Unable to delete file named '{file_name}'."),
                    "reason": format!("Error occured while deleting: {error}")
                });
            }
        }

        return Ok(
            HttpResponse::Ok()
                .content_type("application/json")
                .json(response)
        );
    }

    response = json!({
        "message": format!("Unable to delete file named '{file_name}'."),
        "reason": "File not found."
    });

    Ok(
        HttpResponse::Ok()
            .content_type("application/json")
            .json(response)
    )
}

fn check_if_file_exists(file_name_argument: &String) -> Result<String, Error> {
    let mut found_file = String::from("false");
    let entries = std::fs::read_dir("./uploads")?;

    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name_string = file_name.to_string_lossy().to_string();

        found_file = if &file_name_string == file_name_argument {
            "true".to_string()
        } else {
            continue;
        };
    }

    Ok(found_file)
}

#[get("/download-file")]
pub async fn download_file(query_params: web::Query<QueryParams>) -> Result<NamedFile, Error> {
    let file_name = &query_params.file_name.to_string();
    let mut path: PathBuf = PathBuf::new();

    path.push("./uploads/");
    path.push(file_name);

    Ok(NamedFile::open(path)?)
}

pub async fn upload_file(mut payload: Multipart, _request: HttpRequest) -> Result<HttpResponse, Error> {

    // 1) Receive file.
    // 2) Check if request was "legal".
    // 3) Save the file locally.
    //  3.1) Check if local directory exists.
    //  3.2) Create the directory if needed.
    //  3.3) Create a file.
    //  3.4) Push chunks into the file.

    let mut files_created = Vec::new();
    let mut file_count = 0;

    while let Some(item) = payload.next().await {
        file_count = file_count + 1;
        let mut field = item?;

        let filetype: Option<&Mime> = field.content_type();
        if filetype.is_none() { continue; }

        let filename = field.content_disposition()
            .get_filename()
            .unwrap_or("unknown");

        let read_result = fs::read_dir("./uploads")
            .await
            .map_err(|error| { error });

        match read_result {
            Err(err) => {
                // Checking if directory was not found, which is identified as code "3"
                if err.raw_os_error() == Some(3) {
                    let _ = fs::create_dir("./uploads")
                        .await
                        .map_err(|error| {
                            eprintln!("Error creating directory: {:?}", error);
                        });
                }
            },
            _ => println!("Application has existing 'uploads' directory")
        }

        let filepath = format!("./uploads/{filename}");

        let mut saved_file = fs::File::create(filepath).await.unwrap();
        while let Some(chunk) = field.next().await {
            let chunk = chunk.unwrap();
            let _ = saved_file.write_all(&chunk).await.unwrap();
        }

        files_created.push(saved_file);
    }

    let message = format!("loaded {} out of {} file(s)", files_created.len(), file_count);
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
