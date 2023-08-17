use std::net::TcpStream;
use serde_json::Value;
use std::io::prelude::*;
use std::fs;
use std::io;

// Which ever file is using process_file needs to make sure that they also import FileObject
use super::FileObject;

// Handling communication between host and other devices.
// Currently limited to sending text files.
pub fn handle_sending_file(file_details: Value) -> (&'static str, &'static str) {
    // let path = file_json["path"].as_str().expect("Path not found");
    let name = file_details["name"].as_str().expect("Name not found");
    let file_name = format!("{name}");
    println!("read file named {}", file_name);

    let file_string = read_file_utf16(&file_name).expect("Couldn't read file");
    let file_object = FileObject::new(String::from(file_name), file_string);
    println!("file contents {:?}", file_object);

    let response: Result<(), io::Error> = send_file_to_address("127.0.0.1:7878", file_object);
    let (mut status, mut message) = ("HTTP/1.1 200 OK", "Data was successfully delivered.");

    // Successful response when file has been sent successfully
    match response {
        Ok(res) => {
            println!("Data was successfully delivered.\r{:?}", res);
        }
        Err(error) => {
            println!("Error occured:\n{}", error);
            status = "HTTP/1.1 500 Internal Server Error";
            message = "Couldn't send the file";
        }
    }

    (status, message)
}

fn read_file_utf16(filename: &str) -> Result<String, io::Error> {
    let mut file = fs::File::open(filename).expect("Couldn't open the file");
    let metadata = file.metadata().expect("Couldn't read metadata of the file");
    let file_size = metadata.len() as usize;

    let mut contents = vec![0u8; file_size];
    file.read_exact(&mut contents).expect("Couldn't read the file");

    let file_str = String::from_utf8_lossy(&contents).into_owned();

    // Convert the bytes to a UTF-16 encoded string
    let utf16_string = format_into_utf16(file_str).expect("Couldn't convert file to UTF-16");
    Ok(utf16_string)
}

fn format_into_utf16(file_content: String) -> Result<String, io::Error> {
    // Filter map is used to filter out invalid unicodes which result in None from char::from_u32
    let utf16_content: String = file_content
        .encode_utf16()
        .filter_map(|c| char::from_u32(c as u32))
        .collect();

    Ok(utf16_content)
}

fn send_file_to_address(target_address: &'static str, file: FileObject) -> io::Result<()> {
    let mut stream = TcpStream::connect(target_address).expect("Couldn't create TCP connection");

    let serialized_data = file.serialize();
    let data = serialized_data.as_bytes();

    println!("sending this: {}", serialized_data);

    stream.write_all(data).expect("Couldn't buffer or validate the data");
    stream.flush().expect("Couldn't finish sending the data");

    Ok(())
}