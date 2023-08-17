use server::setup_server;
mod server;

fn main() {
    // let ip_address: Ipv4Addr = get_ip().unwrap();
    // let subnet_mask: Ipv4Addr = Ipv4Addr::from_str("255.255.255.255").unwrap();

    // let broadcast_address: Option<Ipv4Addr> = calculate_broadcast_address(ip_address, subnet_mask);

    // let devices: Option<_> = match broadcast_address {
    //     Some(address) => fetch_device_ips_from_broadcast(address),
    //     _ => None
    // };

    setup_server();
}