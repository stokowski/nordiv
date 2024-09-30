use reqwest;
use serde_json::Value;
use std::net::Ipv4Addr;
use std::collections::HashMap;

pub struct ServerInfo {
    pub hostname: String,
    pub ip_address: Ipv4Addr,
    pub matched_subnets: Vec<(Ipv4Addr, u8)>,
    pub missed_subnets: Vec<(Ipv4Addr, u8)>,
}

pub struct ApiResult {
    pub total_servers: usize,
    pub online_servers: usize,
    pub server_info: Vec<ServerInfo>,
    pub grouped_servers: HashMap<(Ipv4Addr, u8), Vec<String>>,
    pub servers_without_matches: Vec<String>,
}

pub async fn fetch_and_process_servers(api_url: &str, subnets: &[(Ipv4Addr, u8)]) -> Result<ApiResult, String> {
    // Make the API call
    let response = reqwest::get(api_url)
        .await
        .map_err(|e| format!("Failed to make API request: {}", e))?;

    let json: Value = response.json()
        .await
        .map_err(|e| format!("Failed to parse JSON response: {}", e))?;

    // Process the server information
    let servers = json.as_array()
        .ok_or_else(|| "Invalid JSON structure: expected an array".to_string())?;

    let total_servers = servers.len();
    let mut online_servers = 0;
    let mut server_info = Vec::new();
    let mut grouped_servers: HashMap<(Ipv4Addr, u8), Vec<String>> = HashMap::new();
    let mut servers_without_matches = Vec::new();

    for server in servers {
        if let (Some(status), Some(hostname), Some(station)) = (
            server["status"].as_str(),
            server["hostname"].as_str(),
            server["station"].as_str(),
        ) {
            if status == "online" {
                online_servers += 1;
                if let Ok(ip) = station.parse::<Ipv4Addr>() {
                    let mut matched_subnets = Vec::new();
                    let mut missed_subnets = Vec::new();

                    for &subnet in subnets {
                        if is_ip_in_subnet(ip, subnet.0, subnet.1) {
                            matched_subnets.push(subnet);
                            grouped_servers.entry(subnet).or_default().push(hostname.to_string());
                        } else {
                            missed_subnets.push(subnet);
                        }
                    }

                    if matched_subnets.is_empty() {
                        servers_without_matches.push(hostname.to_string());
                    }

                    server_info.push(ServerInfo {
                        hostname: hostname.to_string(),
                        ip_address: ip,
                        matched_subnets,
                        missed_subnets,
                    });
                }
            }
        }
    }

    Ok(ApiResult {
        total_servers,
        online_servers,
        server_info,
        grouped_servers,
        servers_without_matches,
    })
}

fn is_ip_in_subnet(ip: Ipv4Addr, subnet_ip: Ipv4Addr, prefix: u8) -> bool {
    let ip_u32 = u32::from(ip);
    let subnet_u32 = u32::from(subnet_ip);
    let mask = !((1 << (32 - prefix)) - 1);
    (ip_u32 & mask) == (subnet_u32 & mask)
}
