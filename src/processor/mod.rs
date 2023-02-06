mod dependency_factory;
mod production_dependency_factory;

pub use dependency_factory::DependencyFactory;
pub use production_dependency_factory::ProductionDependencyFactory;

use crate::arguments;

use std::error::Error;
use string_error::into_err;

/// CLI processor.
pub struct Processor {
    dependency_factory: Box<dyn dependency_factory::DependencyFactory>,
}

impl Processor {
    /// Creates a new Processor with the specified dependency factory.
    pub fn new(dependency_factory: Box<dyn dependency_factory::DependencyFactory>) -> Self {
        Self { dependency_factory }
    }

    /// Performs CLI processing.
    pub fn run(&self, cli: arguments::Cli) -> Result<(), Box<dyn Error>> {
        let data_storage = self.dependency_factory.make_data_storage();

        match cli.command {
            arguments::WeatherCommand::Provider(provider) => match provider.command {
                arguments::ProviderSubcommand::Add(provider) => {
                    match data_storage.get_provider(&provider.name) {
                        Ok(provider) => {
                            return Err(into_err(format!(
                                "Provider '{}' already exists",
                                provider.name
                            )))
                        }
                        Err(_) => match data_storage.add_provider(&provider) {
                            Ok(_) => {
                                println!("Successfully added '{}' provider", provider.name)
                            }
                            Err(e) => {
                                return Err(into_err(format!("Failed to add new provider ({e})")))
                            }
                        },
                    }
                }
                arguments::ProviderSubcommand::Remove { name } => {
                    match data_storage.remove_provider(&name) {
                        Ok(_) => println!("Successfully removed '{name}' provider"),
                        Err(e) => {
                            return Err(into_err(format!(
                                "Failed to remove provider '{name}' ({e})"
                            )))
                        }
                    }
                }
                arguments::ProviderSubcommand::Show { name } => match name {
                    Some(name) => match data_storage.get_provider(&name) {
                        Ok(provider) => println!("{}", provider),
                        Err(e) => {
                            return Err(into_err(format!("Provider '{name}' not found ({e})")))
                        }
                    },
                    None => match data_storage.get_all_providers() {
                        Ok(all_providers) => {
                            for e in all_providers {
                                println!("{e}\n");
                            }
                        }
                        Err(e) => return Err(into_err(format!("Failed to get providers ({e})"))),
                    },
                },
            },

            arguments::WeatherCommand::Get {
                address,
                date,
                provider_name,
            } => {
                let provider_config = match data_storage.get_provider(&provider_name) {
                    Ok(provider) => provider,
                    Err(e) => {
                        return Err(into_err(format!(
                            "Provider '{provider_name}' not found ({e})"
                        )));
                    }
                };

                let provider_factory = self.dependency_factory.make_provider_factory();
                let worker = provider_factory.make_provider(&provider_config);

                match worker.get_weather(address, date) {
                    Ok(weather) => println!("{weather}"),
                    Err(e) => println!("{e}"),
                }
            }
        }

        Ok(())
    }
}
