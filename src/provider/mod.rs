mod production_provider_factory;
mod provider_factory;

mod accu_weather;
mod open_weather;

use std::error::Error;
use std::fmt;

pub use production_provider_factory::ProductionProviderFactory;
pub use provider_factory::ProviderFactory;

/// Weather condition info.
///
/// All fields MUST be optional. Provider is allowed to fill in available info.
pub struct WeatherInfo {
    /// Textual weather description
    pub description: Option<String>,

    /// Temperature in Celsius
    pub temperature: Option<f64>,

    /// Humidity in percent
    pub humidity: Option<i64>,

    /// Pressure in Pascal
    pub pressure: Option<i64>,
}

impl fmt::Display for WeatherInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\n-> Temperature: {}\n-> Humidity: {}\n-> Pressure: {}",
            self.description
                .as_ref()
                .unwrap_or(&String::from("unknown weather description")),
            self.temperature
                .map(|e| e.to_string())
                .unwrap_or_else(|| "unknown".to_string()),
            self.humidity
                .map(|e| e.to_string())
                .unwrap_or_else(|| "unknown".to_string()),
            self.pressure
                .map(|e| e.to_string())
                .unwrap_or_else(|| "unknown".to_string()),
        )
    }
}

/// Weather provider
pub trait Provider {
    /// Performs weather condition discovery
    fn get_weather(
        &self,
        address: String,
        date: Option<String>,
    ) -> Result<WeatherInfo, Box<dyn Error>>;
}
