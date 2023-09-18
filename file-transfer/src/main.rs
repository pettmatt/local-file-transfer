mod server;
use server::setup_server;

#[macro_use]
extern crate serde_json;

fn main() {
    let _ = setup_server();
}