use crate::processor::dependency_factory;

use crate::provider::ProductionProviderFactory;
use crate::provider::ProviderFactory;

use crate::arguments::DataStorage;
use crate::arguments::ProductionDataStorage;

/// Processor dependencies factory to be used in production.
pub struct ProductionDependencyFactory;

impl dependency_factory::DependencyFactory for ProductionDependencyFactory {
    /// Makes production data storage.
    fn make_data_storage(&self) -> Box<dyn DataStorage> {
        Box::new(ProductionDataStorage)
    }

    /// Makes production provider factory
    fn make_provider_factory(&self) -> Box<dyn ProviderFactory> {
        Box::new(ProductionProviderFactory)
    }
}
