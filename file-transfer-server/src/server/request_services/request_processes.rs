use actix_web::{web, Error};
use tokio::io::AsyncWriteExt;
use tokio::fs;
use tokio::fs::File;
use std::path::Path;
use futures_util::StreamExt;

use crate::server::request_services::SimpleQueryParams;
use crate::server::request_services::NamedFile;
use crate::server::request_services::Multipart;
use crate::server::request_services::PathBuf;
use crate::server::request_services::Mime;

pub async fn check_if_file_exists(file_name_argument: &String, owner_name: &String) -> Result<(bool, PathBuf), Error> {
    let mut _found_file = false;
    let mut path: PathBuf = PathBuf::new();

    // Constructing a path to the file
    path.push("./uploads/");

    if !owner_name.is_empty() {
        path.push(format!("{}/", owner_name));
    }

    path.push(file_name_argument);

    // If file is opened successfully the file is accessable
    match NamedFile::open(&path) {
        Ok(_value) => _found_file = true,
        Err(_error) => _found_file = false
    }

    Ok((_found_file, path))
}

pub async fn get_files_from_dir(dir: &PathBuf) -> std::io::Result<Vec<serde_json::Value>> {
    check_and_create_file_dir().await;

    // Collect file details in a single list, not data included
    let mut files: Vec<serde_json::Value> = Vec::new();
    let mut directories: Vec<PathBuf> = Vec::new();

    directories.push(dir.to_owned());

    while let Some(path) = directories.pop() {
        for entry in std::fs::read_dir(&path)? {
            let entry = entry?;
            let entry_path = entry.path();

            // If the target is not a file, but a directory,
            // add the path to directory list.
            if entry_path.is_dir() {
                directories.push(entry_path);
            }
            
            // If file is not a directory, scrape the details and push them to the "files" variable.
            // Details include: name, size, type and the owner.
            // Only owner can delete the file, through the API.
            // Owner is indicated by the name of the directory inside of uploads directory.
            else {
                // file_name function returns an OsString, which needs to be converted to a string.
                let file_name = entry.file_name().to_string_lossy().to_string(); 
                // There is possibility that the path contains "\\", which can result in "file not found" error.
                let corrected_path = replace_characters_in_path(entry.path(), "\\", "/");
                let corrected_path_string = corrected_path.to_string_lossy().to_string();

                let metadata = fs::metadata(corrected_path_string).await?;
                let file_size = metadata.len();

                let file_extension = Path::new(&corrected_path)
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or("unknown");

                // Possible media types:
                // https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types
                let mime_type = match file_extension {
                    "avi" => "video/x-msvideo",
                    "doc" => "application/msword",
                    "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
                    "gif" => "image/gif",
                    "html" | "htm" => "text/html",
                    "jpg" | "jpeg" => "image/jpeg",
                    "js" => "text/javascript",
                    "json" => "application/json",
                    "mp3" => "audio/mpeg",
                    "mp4" => "video/mp4",
                    "png" => "image/png",
                    "pdf" => "application/pdf",
                    "php" => "application/x-httpd-php",
                    "ppt" => "application/vnd.ms-powerpoint",
                    "sh" => "application/x-sh",
                    "txt" => "text/plain",
                    "rar" => "application/vnd.rar",
                    "tar" => "application/x-tar",
                    "zip" => "application/zip",
                    _ => "application/octet-stream",
                };

                // Check if path contains a directory that contains owner's name.
                // Example "./uploads/{owner}/{file_name}" print out owner, else "uploads"
                let owner = match &entry_path.parent() {
                    Some(parent) => {
                        let mut owner = String::from("uploads");

                        if let Some(dir_name) = parent.file_name() {
                            owner = dir_name.to_string_lossy().to_string();
                        }

                        owner
                    }
                    _ => String::from("uploads")
                };

                let file_details = json!({
                    "name": file_name,
                    "size": file_size,
                    "type": mime_type,
                    "owner": owner
                });

                files.push(file_details);
            }
        }
    }

    Ok(files)
}

fn replace_characters_in_path(original_path: PathBuf, replace_character: &str, replace_with: &str) -> PathBuf {
    let path = original_path.to_string_lossy().to_string();
    let corrected_string = path.replace(&replace_character, &replace_with);
    let corrected_path = PathBuf::from(corrected_string);
    corrected_path
}

pub async fn process_payload(mut payload: Multipart, query_params: web::Query<SimpleQueryParams>) -> Result<(Vec<File>, i32), Error> {

    // 1) Receive file.
    // 2) Check if request was "legal".
    // 3) Save the file locally.
    //  3.1) Check if local directory exists.
    //  3.2) Create the directory if needed.
    //  3.3) Create a file.
    //  3.4) Push chunks into the file.

    let mut files_created = Vec::new();
    let mut file_count = 0;

    while let Some(field) = payload.next().await {
        let mut field = field?;
        file_count = file_count + 1;

        let filetype: Option<&Mime> = field.content_type();
        if filetype.is_none() {
            continue;
        }

        let filename = field.content_disposition()
            .get_filename()
            .unwrap_or("unnamedfile");

        check_and_create_file_dir().await;

        // check_if_directory_is_created(read_result, &String::from("./uploads"));

        let mut _filepath = String::new();

        // Checking if query parameters contain an username and if the program needs to use personalized path.
        if query_params.username.is_empty() {
            _filepath = format!("./uploads/{filename}");
        }
        else {
            let username = query_params.username.to_string();
            let personal_path = format!("./uploads/{username}");
            let personal_read_result = fs::read_dir(&personal_path)
                .await
                .map_err(|error| { error });

            match personal_read_result {
                Err(err) => {
                    // Checking if directory was not found, which is identified as code "3"
                    if err.raw_os_error() == Some(3) {
                        let _ = fs::create_dir(&personal_path)
                            .await
                            .map_err(|error| {
                                eprintln!("Error creating directory: {:?}", error);
                            });
                    }
                },
                _ => println!("Application has existing '{personal_path}' directory")
            }

            // check_if_directory_is_created(personal_read_result, &personal_path);
            _filepath = format!("{personal_path}/{filename}");
        }

        let mut saved_file = fs::File::create(_filepath).await.unwrap();

        // Finally push the chunks to the file
        while let Some(chunk) = field.next().await {
            let chunk = chunk.unwrap();
            let _ = saved_file.write_all(&chunk).await.unwrap();
        }

        files_created.push(saved_file);
    }

    Ok((files_created, file_count))
}

pub fn validate_username(username: &String) -> String {
    // Every file will include the owner name, even if the owner is the root directory "uploads".
    // Delete request is expecting to get username as a parameter, but this will introduce a problem,
    // which will result in "not found" error, if the username is used as is, which is why this function
    // is needed to filter out unwanted username, when there is no real owner of the file.
    if username == "uploads" {
        String::from("")
    } else {
        username.to_string()
    }
}

async fn check_and_create_file_dir() {
    // Creating and verifying the API uses specific directory structure.
    // 1) personal path: ./uploads/{username}/{filename}
    // 2) root path: ./uploads/{filename}
    let read_result = fs::read_dir("./uploads")
        .await
        .map_err(|error| { error });

    match read_result {
        Err(err) => {
            // Checking if directory was not found, which is identified as code "3"
            if err.raw_os_error() == Some(3) || err.raw_os_error() == Some(2) {
                let _ = fs::create_dir("./uploads")
                    .await
                    .map_err(|error| {
                        eprintln!("Error creating directory: {:?}", error);
                    });
            }
        },
        _ => println!("Application has existing './uploads' directory")
    }
}

// async fn check_if_directory_is_created(read_result: Result<fs::ReadDir, std::io::Error>, create_dir_path: &String) {
//     println!("CHECK IF PATH EXISTS {:?}", &create_dir_path);
// }