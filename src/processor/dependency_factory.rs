use crate::arguments::DataStorage;
use crate::provider::ProviderFactory;

/// Processor dependencies factory.
///
/// Allows easy storage and provider implementation switching.
pub trait DependencyFactory {
    /// Makes implementation-specific data storage.
    fn make_data_storage(&self) -> Box<dyn DataStorage>;

    /// Makes implementation-specific provider factory.
    fn make_provider_factory(&self) -> Box<dyn ProviderFactory>;
}
