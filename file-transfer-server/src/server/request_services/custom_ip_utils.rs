use std::net::{SocketAddrV4, IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::str::FromStr;
use std::io::{Read, Write};
use ipnetwork::Ipv4Network;
use socket2::{Socket, Domain, Type};

#[allow(dead_code)]
pub fn get_ip() -> Option<Ipv4Addr> {
    // use std::io::Error;
    use std::process::Command;

    let output = match Command::new("wmic")
        .args(&["nicconfig", "where", "IPEnabled=TRUE", "get", "IPAddress"])
        .output() 
    {
        Ok(output) => output,
        Err(_) => return None
    };

    if !output.status.success() {
        return None
    }

    let output_str = match String::from_utf8(output.stdout) {
        Ok(s) => s,
        Err(_) => return None
    };

    let lines: Vec<&str> = output_str.lines().collect();
    if lines.len() >= 2 {
        let ip = lines[1].trim();

        // Because the ip address includes some unwanted 
        // characters we need to trim them out
        let matches: &[_] = &['{', '}', '"'];
        let trimmed_ip = ip.trim_matches(matches);

        if let Ok(ip_address) = Ipv4Addr::from_str(trimmed_ip) {
            return Some(ip_address);
        }
    }

    None
}

#[allow(dead_code)]
pub fn calculate_broadcast_address(ip: Ipv4Addr, subnet_mask: Ipv4Addr) -> Option<Ipv4Addr> {
    let ip_u32 = u32::from(ip);
    let subnet_mask_u32 = u32::from(subnet_mask);

    let network_address_u32 = ip_u32 & subnet_mask_u32;
    let host_section_u32 = !subnet_mask_u32;
    let broadcast_address_u32 = network_address_u32 | host_section_u32;
    let broadcast_address: Ipv4Addr = Ipv4Addr::from(broadcast_address_u32);

    // Add some checks for making sure that the final result does include the broadcast address

    Some(broadcast_address)
}

#[allow(dead_code)]
pub fn fetch_device_ips_from_broadcast(broadcast_ip: Ipv4Addr) -> Option<Vec<SocketAddrV4>> {
    // Create UDP socket
    let socket = Socket::new(Domain::IPV4, Type::DGRAM, None).expect("Failed to create socket");

    let broadcast_socket = SocketAddr::new(IpAddr::from_str("0.0.0.0").unwrap(), 0);
    socket.bind(&broadcast_socket.into()).expect("Failed to bind socket");

    // Enable BROADCAST option to allow sending to the broadcast address
    socket.set_broadcast(true).expect("Failed to set socket broadcast option");

    // Get the network prefix from the broadcast address
    let network_prefix = broadcast_ip.octets();
    let network_prefix: Ipv4Addr = Ipv4Addr::new(network_prefix[0], network_prefix[1], network_prefix[2], 0);
    let network = Ipv4Network::new(network_prefix, 24).expect("Invalid network prefix");

    // Variable that will store the possible IPs that needs to be checked
    let mut ip_list: Vec<SocketAddrV4> = Vec::new();

    // Iterate through all possible host addresses in the network
    for host in network.iter() {
        // Skip the broadcast address and the network address itself
        if host == network.network() || host == broadcast_ip {
            continue;
        }

        // Add the host address to the list
        let socket_address: SocketAddrV4 = SocketAddrV4::new(host, 0).into();
        ip_list.push(socket_address);
    }

    let (error_counter, usable_addresses) = check_connectivity_with_ip_addresses(ip_list);

    println!("Error counter: {}", error_counter);
    Some(usable_addresses)
}

fn check_connectivity_with_ip_addresses(ip_list: Vec<SocketAddrV4>) -> (u8, Vec<SocketAddrV4>) {
    let mut error_counter: u8 = 0;
    let mut usable_addresses: Vec<SocketAddrV4> = Vec::new();

    // Check which IPs are possibly usable.
    for address in ip_list {
        // Handling panic in case connection fails, which it will
        match TcpStream::connect(address) {
            Ok(mut stream) => {
                let request = format!("GET / HTTP/1.1\r\nHost: {:?}\r\nConnection: close\r\n\r\n", address);
                stream.write_all(request.as_bytes()).expect("Failed to send request");

                let mut response = String::new();
                stream.read_to_string(&mut response).expect("Failed to read response");

                usable_addresses.push(address);
                println!("RESPONSE:\n{}", response);
            }
            Err(_error) => {
                error_counter += 1;
            }
        };
    }

    (error_counter, usable_addresses)
}