// Might be used. Maybe not
fn listening_socket(address: Ipv4Addr, port: u8) {
    let full_address = address;
    let listener = TcpListener::bind(full_address.to_string()).expect("Failed to bind address to a socket");

    println!("Listening on {}", full_address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(error) => {
                eprintln!("Error while processing connection:\n{}", error);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    while let Ok(size) = stream.read(&mut buffer) {
        if size == 0 {
            break; // Connection was closed
        }

        stream.write(&buffer[..size]).expect("Failed to write data to the client");
    }
}