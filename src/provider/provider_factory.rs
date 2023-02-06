use crate::arguments;
use crate::provider;

/// Weather provider factory
pub trait ProviderFactory {
    /// Makes provider based on specified config
    fn make_provider(&self, config: &arguments::WeatherProvider) -> Box<dyn provider::Provider>;
}
