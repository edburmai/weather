use crate::provider::{Provider, WeatherInfo};

//use chrono::{DateTime, Duration, Utc};
use reqwest::StatusCode;
use std::error::Error;
use string_error::{into_err, static_err};

static LOCATION_API_URL: &str = "http://dataservice.accuweather.com/locations/v1/cities/search";
static CURRENT_CONDITION_API_URL: &str = "http://dataservice.accuweather.com/currentconditions/v1";

pub struct AccuWeather {
    api_key: String,
}

impl AccuWeather {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

impl Provider for AccuWeather {
    fn get_weather(
        &self,
        address: String,
        _date: Option<String>,
    ) -> Result<WeatherInfo, Box<dyn Error>> {
        let response = reqwest::blocking::get(format!(
            "{}?apikey={}&q={}",
            LOCATION_API_URL, self.api_key, address
        ))
        .map_err(|e| into_err(format!("Request failed ({e})")))
        .and_then(|r| {
            if r.status() != StatusCode::OK {
                return Err(into_err(format!("Request failed ({})", r.status())));
            }
            Ok(r)
        })?;

        let data = response
            .json::<serde_json::Value>()
            .map_err(|e| into_err(format!("Failed to parse response data ({e})")))?;

        let location_key = match data
            .as_array()
            .and_then(|array| array.get(0))
            .and_then(|loc| loc["Key"].as_str())
        {
            Some(key) => key,
            None => return Err(static_err("Unknown location")),
        };

        let response = reqwest::blocking::get(format!(
            "{}/{}?apikey={}",
            CURRENT_CONDITION_API_URL, location_key, self.api_key
        ))
        .map_err(|e| into_err(format!("Request failed ({e})")))
        .and_then(|r| {
            if r.status() != StatusCode::OK {
                return Err(into_err(format!("Request failed ({})", r.status())));
            }
            Ok(r)
        })?;

        let data = response
            .json::<serde_json::Value>()
            .map_err(|e| into_err(format!("Failed to parse response data ({e})")))?;

        let mut info = WeatherInfo {
            description: None,
            temperature: None,
            humidity: None,
            pressure: None,
        };

        let condition = match data.as_array().and_then(|array| array.get(0)) {
            Some(key) => key,
            None => return Err(static_err("No weather condition received")),
        };

        if let Some(description) = condition["WeatherText"].as_str() {
            info.description = Some(description.to_string());
        }

        if let Some(temperature) = condition["Temperature"]["Metric"]["Value"].as_f64() {
            info.temperature = Some(temperature);
        }

        Ok(info)
    }
}
