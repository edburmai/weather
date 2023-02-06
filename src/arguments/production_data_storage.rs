use crate::arguments::{data_storage, WeatherProvider};

use std::env;
use std::error::Error;
use std::fs::OpenOptions;
use std::io;
use std::path::PathBuf;

use string_error::into_err;

fn get_config_path() -> PathBuf {
    let mut path = match home::home_dir() {
        Some(path) => path,
        None => env::temp_dir(),
    };

    path.push(".weather_providers.data");
    path
}

fn save_providers(providers: Vec<WeatherProvider>) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(get_config_path())
        .map_err(|e| {
            into_err(format!(
                "Failed to open providers file while saving config ({e})"
            ))
        })?;

    let value = serde_json::to_value(providers)
        .map_err(|e| into_err(format!("Failed to serialize providers config ({e})")))?;

    serde_json::to_writer(&file, &value)
        .map_err(|e| into_err(format!("Failed to save providers config ({e})")))
}

/// Data storage to be used in production
pub struct ProductionDataStorage;

impl data_storage::DataStorage for ProductionDataStorage {
    fn get_all_providers(&self) -> Result<Vec<WeatherProvider>, Box<dyn Error>> {
        let file = match OpenOptions::new().read(true).open(get_config_path()) {
            Ok(file) => file,
            Err(e) => {
                if e.kind() == io::ErrorKind::NotFound {
                    // Absence of the config means empty config.
                    return Ok(Vec::<WeatherProvider>::new());
                }
                return Err(into_err(format!(
                    "Failed to open providers file while reading config ({e})"
                )));
            }
        };

        let providers: Vec<WeatherProvider> = match serde_json::from_reader(&file) {
            Ok(providers) => providers,
            Err(e) => {
                if e.is_eof() {
                    // Empty config file equals to empty config.
                    return Ok(Vec::<WeatherProvider>::new());
                } else if e.is_data() || e.is_syntax() {
                    return Err(into_err(format!("Broken config data/syntax ({e})")));
                } else {
                    return Err(into_err(format!("Failed to parse providers config ({e})")));
                }
            }
        };

        Ok(providers)
    }

    fn get_provider(&self, name: &str) -> Result<WeatherProvider, Box<dyn Error>> {
        match self.get_all_providers() {
            Ok(mut providers) => {
                if let Some(position) = providers.iter().position(|e| e.name == *name) {
                    return Ok(providers.swap_remove(position));
                }
                return Err(into_err("Not found".to_string()));
            }
            Err(e) => Err(into_err(format!("Failed to get providers list ({e})"))),
        }
    }

    fn add_provider(&self, provider: &WeatherProvider) -> Result<(), Box<dyn Error>> {
        let mut providers = self
            .get_all_providers()
            .map_err(|e| into_err(format!("Failed to get providers list ({e})")))?;

        providers.push(provider.clone());
        save_providers(providers)
    }
    fn remove_provider(&self, name: &str) -> Result<(), Box<dyn Error>> {
        let mut providers = self
            .get_all_providers()
            .map_err(|e| into_err(format!("Failed to get providers list ({e})")))?;

        match providers.iter().position(|e| e.name == *name) {
            Some(pos) => providers.remove(pos),
            None => return Err(into_err("Not found".to_string())),
        };

        save_providers(providers)
    }
}
