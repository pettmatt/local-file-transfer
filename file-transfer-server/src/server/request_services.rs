use actix_web::{web, get, delete, Error, HttpResponse, HttpRequest, Responder};
use actix_multipart::Multipart;
use actix_files::NamedFile;
use std::path::PathBuf;
use mime::Mime;

use request_processes::{get_files_from_dir, check_if_file_exists, process_payload, validate_username};

mod request_processes;
pub mod custom_ip_utils;

#[get("/")]
pub async fn frontpage() -> impl Responder {
    HttpResponse::Ok().body("Hello world! This is the API for local file transfer.")
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

#[get("/files")]
pub async fn get_local_files() -> Result<HttpResponse, Error> {

    let files = get_files_from_dir(&PathBuf::from("./uploads")).await?;

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
pub struct QueryParams {
    file_name: String,
    username: String,
}

#[delete("/files")]
pub async fn remove_local_file(query_params: web::Query<QueryParams>) -> Result<HttpResponse, Error> {

    // 1) Get name of the file from request params.
    // 2) Search file with the same name and delete it.

    let file_name = &query_params.file_name.to_string();
    let username = &query_params.username;

    let username = validate_username(username);
    let (found_file, file_path) = check_if_file_exists(&file_name, &username).await.unwrap();

    let mut response = json!({
        "message": format!("File '{}' deleted", file_name)
    });

    if found_file {
        let file_deletion = std::fs::remove_file(file_path);

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

#[get("/download")]
pub async fn download_file(request: HttpRequest, query_params: web::Query<QueryParams>) -> HttpResponse {
    let file_name = &query_params.file_name.to_string();
    let owner_name = &query_params.username.trim().to_string();
    
    // Path should be either "./uploads/{owner_name}/{file_name}" or "./uploads/{file_name}".
    let mut path: PathBuf = PathBuf::new();

    path.push("./uploads/");

    if !owner_name.is_empty() {
        // When user uploads files without a name it's uploaded as "uploads"
        if owner_name != "uploads" {
            path.push(format!("{}/", owner_name));
        }
    }

    path.push(file_name);
    let file = NamedFile::open_async(path).await.unwrap();

    file.into_response(&request)
}

#[derive(serde::Deserialize, Debug)]
pub struct SimpleQueryParams {
    username: String
}

// POST /send
pub async fn upload_file(payload: Multipart, query_params: web::Query<SimpleQueryParams>) -> Result<HttpResponse, Error> {

    let (files_created, file_count) = process_payload(payload, query_params).await?;

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