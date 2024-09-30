use clap::Parser;

#[derive(Parser)]
#[command(
    name = "nordiv",
    version = "0.1.0",
    author = "Rafał Stokowski <rafal@stokowski.net>",
    about = "Divide NordVPN servers into subnets"
)]
pub struct Cli {
    /// Sets a custom config file
    #[arg(short, long)]
    pub config: Option<String>,

    /// Sets the API URL (used only if config is not provided)
    #[arg(short = 'u', long = "api-url")]
    pub api_url: Option<String>,

    /// Sets the subnet (used only if config is not provided)
    #[arg(short = 's', long = "subnet")]
    pub subnet: Option<String>,

    /// Sets the new prefix (used only if config is not provided)
    #[arg(short = 'p', long = "new-prefix")]
    pub new_prefix: Option<i32>,

    /// Sets the level of logging (used only if config is not provided)
    #[arg(short = 'l', long = "log-level")]
    pub log_level: Option<String>,

    /// Sets the log file (used only if config is not provided)
    #[arg(short = 'f', long = "log-file")]
    pub log: Option<String>,
}

impl Cli {
    pub fn new() -> Self {
        Cli::parse()
    }
}
