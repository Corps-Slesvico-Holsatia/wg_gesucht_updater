# wg_gesucht_updater
Shared flat offers bumper for wg-gesucht.de

## Installation
You need to have a [Rust](https://www.rust-lang.org/) toolchain installed.
```commandline
$ git clone https://github.com/Corps-Slesvico-Holsatia/wg_gesucht_updater.git
$ cd wg_gesucht_updater
$ cargo build --release
```
You'll find the built binary under `target/release/wg_gesucht_updater{,.exe}`.

## Usage
The program has two operation modes:

### CLI mode
In *CLI mode* you can pass the credentials and offer IDs via command line arguments:
```commandline
$ wg_gesucht_updater cli --user-name=your@user.name --password=yourSecretPassword <ACTION> <id> [<id>...]
```
The supported actions are `bump` to bump offers, `activate` to activate offers and `deactivate` to deactivate offers.

### Config file mode
In *config file mode* you can pass the path to a [TOML](https://toml.io/en/) configuration file:
```commandline
$ wg_gesucht_updater config-file /etc/wg-gesucht.toml
```
The configuration file is expected to have the following format:
```toml
user_name = "your@user.name"
password = "yourSecretPassword"
timeout_sec = 10
user_agent = "your_preferred_user_agent_string"
bump = [ <id>, <id>, ... ]
activate = [ <id>, <id>, ... ]
deactivate = [ <id>, <id>, ... ]
```
The fields `timeout_sec` and `user_agent` are optional and have sensible defaults.  
The lists `bump`, `activate` and `deactivate` are optional as well and default to empty lists.