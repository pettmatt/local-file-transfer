use std::net::Ipv4Addr;
use std::str::FromStr;
use ip_utils::{get_ip, calculate_broadcast_address, fetch_devices_from_broadcast};

fn main() {
    let ip_address: Ipv4Addr = get_ip().unwrap();
    let subnet_mask: Ipv4Addr = Ipv4Addr::from_str("255.255.255.255").unwrap();

    let broadcast_address: Option<Ipv4Addr> = calculate_broadcast_address(ip_address, subnet_mask);
    println!("IS LOCAL {}", ip_address.is_private());
    println!("NETWORK {:?}", ip_address);

    let devices: Option<_> = match broadcast_address {
        Some(address) => fetch_devices_from_broadcast(address),
        _ => None
    };

    println!("DEVICES {:?}", devices);
}