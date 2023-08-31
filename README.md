# wg_gesucht_updater
Advertisement bumper for wg-gesucht.de

## Usage
The program has two operation modes:

### CLI mode
In *CLI mode* you can pass the credentials and Ad-IDs via command line arguments:
```commandline
wg_gesucht_updater cli --user-name=your@user.name --password=yourSecretPassword <ad_id> [<ad_id>...]
```

### Config file mode
In *config file mode* you can pass the path to a TOML configuration file:
```commandline
wg_gesucht_updater config-file /etc/wg-gesucht.toml
```
The configuration file is expected to have the following format:
```toml
user_name = "your@user.name"
password = "yourSecretPassword"
ad_ids = [ <id>, <id>, ... ]
```
