use server::setup_server;
mod server;

#[macro_use]
extern crate serde_json;

fn main() {
    setup_server();
}