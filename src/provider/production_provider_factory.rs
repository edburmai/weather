use crate::arguments;
use crate::provider::accu_weather::AccuWeather;
use crate::provider::open_weather::OpenWeather;
use crate::provider::{provider_factory, Provider};

/// Provider factory to be used in production.
pub struct ProductionProviderFactory;

impl provider_factory::ProviderFactory for ProductionProviderFactory {
    /// Makes production weather provider according to specified config
    fn make_provider(&self, config: &arguments::WeatherProvider) -> Box<dyn Provider> {
        match config.provider {
            arguments::Provider::OpenWeather => Box::new(OpenWeather::new(config.api_key.clone())),
            arguments::Provider::AccuWeather => Box::new(AccuWeather::new(config.api_key.clone())),
        }
    }
}
