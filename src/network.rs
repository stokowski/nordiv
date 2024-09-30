// File: network.rs

use std::net::Ipv4Addr;
use std::str::FromStr;

/// Helper function to calculate the network address given an IP address and prefix length.
fn calculate_network_address(ip: Ipv4Addr, prefix: u8) -> Ipv4Addr {
    let ip_u32 = u32::from(ip);
    let network_u32 = ip_u32 & !((1 << (32 - prefix)) - 1);
    Ipv4Addr::from(network_u32)
}

/// Validates if the input is a valid IPv4 subnet in CIDR notation.
pub fn validate_subnet(subnet: &str) -> Result<(Ipv4Addr, u8), String> {
    let parts: Vec<&str> = subnet.split('/').collect();
    if parts.len() != 2 {
        return Err("Invalid subnet format".to_string());
    }

    let ip = Ipv4Addr::from_str(parts[0]).map_err(|_| "Invalid IPv4 address".to_string())?;
    let prefix = parts[1].parse::<u8>().map_err(|_| "Invalid prefix length".to_string())?;
    if prefix > 32 {
        return Err("Prefix length must be between 0 and 32".to_string());
    }

    Ok((ip, prefix))
}

/// Divides the given IPv4 subnet into smaller subnets of the specified new prefix.
/// Returns a vector of new subnets or an error message if the division is not possible.
pub fn divide_subnet(subnet: (Ipv4Addr, u8), new_prefix: u8) -> Result<Vec<(Ipv4Addr, u8)>, String> {
    let (ip, prefix) = subnet;

    if new_prefix <= prefix {
        return Err(format!("New prefix {} must be larger than subnet prefix {}", new_prefix, prefix));
    }

    if new_prefix > 32 {
        return Err("Invalid new prefix for IPv4 network".to_string());
    }

    let mut new_subnets = Vec::new();
    let subnet_size = 1 << (32 - new_prefix);
    let mut current_base: u32 = u32::from(calculate_network_address(ip, prefix));

    for _ in 0..(1 << (new_prefix - prefix)) {
        let network_addr = Ipv4Addr::from(current_base);
        new_subnets.push((network_addr, new_prefix));
        current_base += subnet_size;
    }

    Ok(new_subnets)
}