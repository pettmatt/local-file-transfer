use std::net::{Ipv4Addr, SocketAddrV4};
use std::str::FromStr;

use super::custom_ip_utils::{get_ip, calculate_broadcast_address, fetch_device_ips_from_broadcast};

pub fn get_local_devices() -> Option<Vec<SocketAddrV4>> {
    let ip_address: Ipv4Addr = get_ip().unwrap();
    let subnet_mask: Ipv4Addr = Ipv4Addr::from_str("255.255.255.255").unwrap();

    let broadcast_address: Option<Ipv4Addr> = calculate_broadcast_address(ip_address, subnet_mask);
    println!("NETWORK {:?}", ip_address);

    let devices: Option<Vec<SocketAddrV4>> = match broadcast_address {
        Some(address) => fetch_device_ips_from_broadcast(address),
        _ => None
    };

    devices
}