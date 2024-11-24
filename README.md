# Nordiv

NordVPN subnet servers divider

## Description

`nordiv` is a command-line tool that divides NordVPN servers into subnets. It fetches server information from the NordVPN API, processes it, and groups servers based on specified subnets.

Built with AI with love <3

## Installation

### Prerequisites

- Rust (latest stable version)
- Cargo package manager

### Steps

1. Clone the repository:
   ```
   git clone https://github.com/stokowski/nordiv.git
   cd nordiv
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. The compiled binary will be available at `target/release/nordiv`.

## Usage

You can run `nordiv` either by providing command-line arguments or by using a configuration file.

### Command-line Usage

```
nordiv [OPTIONS]

Options:
  -c, --config <FILE>           Sets a custom config file
  -u, --api-url <URL>           Sets the API URL (used only if config is not provided)
  -s, --subnet <SUBNET>         Sets the subnet (used only if config is not provided)
  -p, --new-prefix <PREFIX>     Sets the new prefix (used only if config is not provided)
  -l, --log-level <LEVEL>       Sets the level of logging (used only if config is not provided)
  -f, --log-file <FILE>         Sets the log file (used only if config is not provided)
  -h, --help                    Print help
  -V, --version                 Print version
```

### Configuration File

You can use a configuration file instead of command-line arguments. Create an INI file with the following structure:

```ini
[DEFAULT]
api_url = https://api.nordvpn.com/v1/servers
subnet = 192.168.0.0/24
new_prefix = 26
log_level = info
log_file = nordiv.log
```

Then run `nordiv` with the `-c` option:

```bash
nordiv -c path/to/your/config.ini
```

### Examples

1. Using command-line arguments:
   ```bash
   nordiv --api-url https://api.nordvpn.com/v1/servers --subnet 192.168.1.0/24 --new-prefix 26 --log-level info --log-file nordiv.log
   ```

2. Using a configuration file:
   ```bash
   nordiv -c nordiv.ini
   ```

### Running in Docker

1. Build Docker container
   ```bash
   docker build -t nordiv .
   ```

2. Run Docker container
   ```bash
   docker run --rm -it -v ${PWD}:/app -- nordiv --api-url https://api.nordvpn.com/v1/servers --subnet 192.168.1.0/24 --new-prefix 26 --log-level debug --log-file /app/nordiv.log
   ```

## Output

The tool will output information about:
- Total and online servers
- Grouped servers by subnet
- Servers without matches
- Detailed server information with matched and missed subnets

Logs will be written to the console or a file based on the configuration.

## License

This project is licensed under the Apache License 2.0. See the [LICENSE](LICENSE) file for details.
