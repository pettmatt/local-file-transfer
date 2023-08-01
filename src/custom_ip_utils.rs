use std::net::Ipv4Addr;
use std::str::FromStr;

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
        // characters we need to trim them out.
        let matches: &[_] = &['{', '}', '"'];
        let trimmed_ip = ip.trim_matches(matches);

        if let Ok(ip_address) = Ipv4Addr::from_str(trimmed_ip) {
            return Some(ip_address);
        }
    }

    None
}

pub fn calculate_broadcast_address(ip: Ipv4Addr, subnet_mask: Ipv4Addr) -> Option<Ipv4Addr> {
    let ip_u32 = u32::from(ip);
    let subnet_mask_u32 = u32::from(subnet_mask);

    let network_address_u32 = ip_u32 & subnet_mask_u32;
    let host_section_u32 = !subnet_mask_u32;
    let broadcast_address_u32 = network_address_u32 | host_section_u32;
    let broadcast_address: Ipv4Addr = Ipv4Addr::from(broadcast_address_u32);

    // Add some checks for making sure that Some does include the broadcast address

    Some(broadcast_address)
}
