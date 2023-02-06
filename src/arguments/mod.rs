extern crate clap;
extern crate serde;
extern crate string_error;

mod data_storage;
mod production_data_storage;

pub use data_storage::DataStorage;
pub use production_data_storage::ProductionDataStorage;

use clap::{Args, Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};

use std::fmt;

/// Root CLI node.
#[derive(Parser)]
#[clap(author, version, about = "about Weather")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: WeatherCommand,
}

/// Root commands.
#[derive(Subcommand)]
pub enum WeatherCommand {
    /// Configure credentials for the weather provider.
    Provider(ProviderCommand),

    /// Show the weather for the provided address.
    Get {
        /// City to get the weather for
        #[clap(forbid_empty_values = true)]
        address: String,

        /// Date  to get the weather for (default is current time)
        #[clap(short, long, forbid_empty_values = true)]
        date: Option<String>,

        /// Weather provider (default configured is used if not specified)
        #[clap(short, long, forbid_empty_values = true)]
        provider_name: String,
    },
}

/// Provider configuration node.
#[derive(Args)]
pub struct ProviderCommand {
    #[clap(subcommand)]
    pub command: ProviderSubcommand,
}

/// Provider configuration commands.
#[derive(Subcommand)]
pub enum ProviderSubcommand {
    /// Add weather provider.
    Add(WeatherProvider),

    /// Add weather provider.
    Remove {
        /// Provider name.
        #[clap(short, long, forbid_empty_values = true)]
        name: String,
    },

    /// Show weather provider.
    Show {
        /// Provider name.
        #[clap(short, long, forbid_empty_values = true)]
        name: Option<String>,
    },
}

/// Provider representation.
#[derive(Clone, Args, Serialize, Deserialize)]
pub struct WeatherProvider {
    #[clap(short, long, forbid_empty_values = true)]
    /// Provider name
    pub name: String,

    #[clap(short, long, value_enum)]
    /// Provider
    pub provider: Provider,

    #[clap(short, long, forbid_empty_values = true)]
    /// Provider API key
    pub api_key: String,
}

/// Supported providers.
#[derive(Clone, ValueEnum, Serialize, Deserialize)]
pub enum Provider {
    /// OpenWeather
    OpenWeather,

    /// AccuWeather
    AccuWeather,
}

impl fmt::Display for WeatherProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Name: {}\nProvider: {}\nAPI key: {}",
            self.name, self.provider, self.api_key
        )
    }
}

impl fmt::Display for Provider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Provider::OpenWeather => "OpenWeather",
                Provider::AccuWeather => "AccuWeather",
            }
        )
    }
}
