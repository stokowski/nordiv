use crate::cli::Cli;
use configparser::ini::Ini;
use std::path::Path;

pub struct Config {
    pub api_url: String,
    pub subnet: String,
    pub new_prefix: i32,
    pub log_level: String,
    pub log_file: String,
}

impl Config {
    pub fn new(cli: &Cli) -> Result<Self, String> {
        if let Some(config_file) = &cli.config {
            Self::from_file(config_file)
        } else {
            Self::from_cli(cli)
        }
    }

    fn from_cli(cli: &Cli) -> Result<Self, String> {
        let api_url = cli.api_url.clone().ok_or("Missing 'api_url'")?;
        let subnet = cli.subnet.clone().ok_or("Missing 'subnet'")?;
        let new_prefix = cli.new_prefix.ok_or("Missing 'new_prefix'")?;
        let log_level = cli.log_level.clone().unwrap_or_else(|| "info".to_string());
        let log_file = cli.log.clone().unwrap_or_else(|| "".to_string());

        Ok(Self {
            api_url,
            subnet,
            new_prefix,
            log_level,
            log_file,
        })
    }

    fn from_file(config_file: &str) -> Result<Self, String> {
        if !Path::new(config_file).exists() {
            return Err(format!("Config file {} does not exist.", config_file));
        }

        let mut config = Ini::new();
        config.load(config_file).map_err(|e| e.to_string())?;

        let api_url = config
            .get("DEFAULT", "api_url")
            .ok_or("Missing 'api_url' in config file")?;
        let subnet = config
            .get("DEFAULT", "subnet")
            .ok_or("Missing 'subnet' in config file")?;
        let new_prefix = config
            .get("DEFAULT", "new_prefix")
            .ok_or("Missing 'new_prefix' in config file")?
            .parse::<i32>()
            .map_err(|_| "'new_prefix' should be an integer")?;
        let log_level = config
            .get("DEFAULT", "log_level")
            .unwrap_or_else(|| "info".to_string());
        let log_file = config
            .get("DEFAULT", "log_file")
            .unwrap_or_else(|| "".to_string());

        Ok(Self {
            api_url,
            subnet,
            new_prefix,
            log_level,
            log_file,
        })
    }
}
