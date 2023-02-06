use crate::provider::{Provider, WeatherInfo};

//use chrono::{DateTime, Duration, Utc};
use reqwest::StatusCode;
use std::error::Error;
use string_error::into_err;

static WEATHER_API_URL: &str = "https://api.openweathermap.org/data/2.5/weather";

pub struct OpenWeather {
    api_key: String,
}

impl OpenWeather {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

impl Provider for OpenWeather {
    fn get_weather(
        &self,
        address: String,
        _date: Option<String>,
    ) -> Result<WeatherInfo, Box<dyn Error>> {
        let response = reqwest::blocking::get(format!(
            "{}?units=metric&q={}&appid={}",
            WEATHER_API_URL, address, self.api_key,
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

        if let Some(weather) = data["weather"].as_array() {
            for e in weather.iter() {
                let descriptions: &mut String = info.description.get_or_insert(String::from(""));
                descriptions.push_str(e["main"].as_str().unwrap_or(""))
            }
        }
        if let Some(temperature) = data["main"]["temp"].as_f64() {
            info.temperature = Some(temperature);
        }
        if let Some(humidity) = data["main"]["humidity"].as_i64() {
            info.humidity = Some(humidity);
        }
        if let Some(pressure) = data["main"]["pressure"].as_i64() {
            info.pressure = Some(pressure);
        }

        Ok(info)
    }
}
