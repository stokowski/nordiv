mod api;
mod cli;
mod config;
mod logger;
mod network;

use tokio;

#[tokio::main]
async fn main() {
    // Parse CLI arguments and config setup remains the same
    let cli = cli::Cli::new();
    let config = match config::Config::new(&cli) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Configuration error: {}", e);
            std::process::exit(1);
        }
    };

    logger::init_logger(Some(&config.log_level), Some(&config.log_file));

    // Log debug information
    log::debug!("API URL: {}", config.api_url);
    log::debug!("Provided subnet: {}", config.subnet);
    log::debug!("New prefix size: {}", config.new_prefix);
    log::debug!("Log Level: {}", config.log_level);
    log::debug!("Log File: {}", config.log_file);

    // Validate and divide subnet
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

    // Fetch and process servers - single pass through data
    let api_result = match api::fetch_and_process_servers(&config.api_url, &new_subnets).await {
        Ok(result) => result,
        Err(err) => {
            log::error!("Failed to fetch and process servers: {}", err);
            return;
        }
    };

    // Log summary information
    log::info!("Total servers: {}", api_result.total_servers);
    log::info!("Online servers: {}", api_result.online_servers);
    
    // Log grouped servers in a single pass
    for (subnet, servers) in &api_result.grouped_servers {
        log::info!("Subnet={:?} Servers={:?}", subnet, servers);
    }

    // Log servers without matches
    log::info!("NoMatchServers={:?}", api_result.servers_without_matches);

    // Log detailed server information in a single pass
    for server in api_result.server_info {
        // Combined logging for matches and misses in one iteration
        log::debug!(
            "Server={} IP={} Matches={:?} Misses={:?}",
            server.hostname,
            server.ip_address,
            server.matched_subnets,
            server.missed_subnets
        );
    }
}

