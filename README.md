# weather
**weather** is a command-line tool for getting weather conditions. 


## Description
The utility is intended to enable users to obtain weather conditions and forecasts from different pre-configured providers. Currently, supported providers are OpenWeather and AccuWeather. 

## Installation

Use the package manager [Cargo](https://doc.rust-lang.org/cargoc) to install **weather**.

```bash
cargo install --git <url> --root <installation path>
```

## Usage

```
USAGE:
    weather <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    get         Show the weather for the provided address
    help        Print this message or the help of the given subcommand(s)
    provider    Configure credentials for the weather provider
```
## Examples
```bash
/// configure provider
weather provider add -n provider_name -p open-weather -a sdfgsdfgkvjqhewgfkjha624h5hfg3 

/// Remove provider
weather provider remove -n provider_name 

/// Show provider
weather provider show -n provider_name

/// Obtain current weather conditions for the given location using the specified provider
weather get Kyiv -p open 
```
## Contributing

The main idea behind the utility is to be extensible. Adding new providers is a pleasure) Contributions are welcomed.
