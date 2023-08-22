use server::setup_server;
mod server;

use std::fs;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

fn main() {
    // setup_server();
    main_test();
}

#[tokio::main]
async fn main_test() {
    let service = make_service_fn(|_connection| {
        async {
            Ok::<_, hyper::Error>(service_fn(handle_request)) 
        }
    });

    let address = "127.0.0.1:7878".parse().unwrap();
    let server = Server::bind(&address).serve(service);

    println!("Server running at http://{}", address);

    if let Err(error) = server.await {
        eprintln!("server error: {}", error);
    }
}

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&hyper::Method::GET, "/") => response_with_file("index.html"),
        (&hyper::Method::GET, "/ping") => response_with_json("{\"message\": \"pong\"}"),
        (&hyper::Method::POST, "/process") => Ok(Response::new(Body::from("Post request, process the body"))),
        _ => response_with_file("404.html"),
    }
}

fn response_with_json(json_string: &str) -> Result<Response<Body>, hyper::Error> {
    let response = Response::builder()
        .header(hyper::header::CONTENT_TYPE, "text/json")
        .body(Body::from(json_string.to_string()))
        .unwrap();

    Ok(response)
}

fn response_with_file (file_path: &str) -> Result<Response<Body>, hyper::Error> {
    let mut contents = String::new();

    match fs::read_to_string(file_path) {
        Ok(file_contents) => {
            contents = file_contents;
        }
        Err(_error) => {
            // If the file couldn't be found, the value is probably JSON object and should be kept as is.
            // println!("The file doesn't exist: {}", _error);
        }
    }

    let response = Response::builder()
        .header(hyper::header::CONTENT_TYPE, "text/html")
        .body(Body::from(contents))
        .unwrap();

    Ok(response)
}