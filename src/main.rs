mod api;
mod cli;
mod config;
mod logger;
mod network;

use tokio;

#[tokio::main]
async fn main() {
    // Parse CLI arguments
    let cli = cli::Cli::new();

    let config = match config::Config::new(&cli) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Configuration error: {}", e);
            std::process::exit(1);
        }
    };

    logger::init_logger(Some(&config.log_level), Some(&config.log_file));

    log::debug!("API URL: {}", config.api_url);
    log::debug!("Provided subnet: {}", config.subnet);
    log::debug!("New prefix size: {}", config.new_prefix);
    log::debug!("Log Level: {}", config.log_level);
    log::debug!("Log File: {}", config.log_file);

    let valid_subnet = match network::validate_subnet(&config.subnet) {
        Ok(subnet) => subnet,
        Err(err) => {
            log::warn!("{}", err);
            return;
        }
    };

    let new_subnets = match network::divide_subnet(valid_subnet, config.new_prefix as u8) {
        Ok(subnets) => subnets,
        Err(err) => {
            log::warn!("{}", err);
            return;
        }
    };

    log::info!("Generated subnets={:?}", new_subnets);

    let api_result = match api::fetch_and_process_servers(&config.api_url, &new_subnets).await {
        Ok(result) => result,
        Err(err) => {
            log::error!("Failed to fetch and process servers: {}", err);
            return;
        }
    };

    log::info!("Total servers: {}", api_result.total_servers);
    log::info!("Online servers: {}", api_result.online_servers);

    for (subnet, servers) in &api_result.grouped_servers {
        log::info!("Subnet={:?} Servers={:?}", subnet, servers);
    }

    log::info!("NoMatchServers={:?}", api_result.servers_without_matches);

    for server in api_result.server_info {
        for subnet in &server.matched_subnets {
            log::debug!(
                "Server={} IP={} Subnet={:?} Status=match",
                server.hostname,
                server.ip_address,
                subnet
            );
        }
        for subnet in &server.missed_subnets {
            log::debug!(
                "Server={} IP={} Subnet={:?} Status=miss",
                server.hostname,
                server.ip_address,
                subnet
            );
        }
    }
}
