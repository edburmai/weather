use crate::arguments::WeatherProvider;
use std::error::Error;

/// Arguments data storage.
pub trait DataStorage {
    /// Get all configured providers.
    fn get_all_providers(&self) -> Result<Vec<WeatherProvider>, Box<dyn Error>>;

    /// Get provider by name.
    fn get_provider(&self, name: &str) -> Result<WeatherProvider, Box<dyn Error>>;

    /// Add a new provider.
    fn add_provider(&self, provider: &WeatherProvider) -> Result<(), Box<dyn Error>>;

    /// Remove the provider by name.
    fn remove_provider(&self, name: &str) -> Result<(), Box<dyn Error>>;
}
