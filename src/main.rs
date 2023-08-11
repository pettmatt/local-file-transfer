use std::net::Ipv4Addr;
use std::str::FromStr;
use custom_ip_utils::{get_ip, calculate_broadcast_address, fetch_device_ips_from_broadcast};
mod custom_ip_utils;
use server::setup_server;
mod server;

fn main() {
    let ip_address: Ipv4Addr = get_ip().unwrap();
    let subnet_mask: Ipv4Addr = Ipv4Addr::from_str("255.255.255.255").unwrap();

    let broadcast_address: Option<Ipv4Addr> = calculate_broadcast_address(ip_address, subnet_mask);
    println!("IS LOCAL {}", ip_address.is_private());
    println!("NETWORK {:?}", ip_address);

    let devices: Option<_> = match broadcast_address {
        Some(address) => fetch_device_ips_from_broadcast(address),
        _ => None
    };

    println!("DEVICES {:?}", devices);

    setup_server();
}