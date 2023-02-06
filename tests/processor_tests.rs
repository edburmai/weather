use mockall::mock;
use string_error::static_err;

use weather::arguments::{
    self, Cli, DataStorage, ProviderCommand, ProviderSubcommand, WeatherCommand, WeatherProvider,
};
use weather::processor::{DependencyFactory, Processor};
use weather::provider::{Provider, ProviderFactory, WeatherInfo};

use std::error::Error;

mock! {
    DataStorage {}
    impl DataStorage for DataStorage {
        fn get_all_providers(&self) -> Result<Vec<WeatherProvider>, Box<dyn Error>>;
        fn get_provider(&self, name: &str) -> Result<WeatherProvider, Box<dyn Error>>;
        fn add_provider(&self, provider: &WeatherProvider) -> Result<(), Box<dyn Error>>;
        fn remove_provider(&self, name: &str) -> Result<(), Box<dyn Error>>;
    }
}

mock! {
    Provider{}
    impl Provider for Provider {
        fn get_weather(
            &self,
            address: String,
            date: Option<String>,
        ) -> Result<WeatherInfo, Box<dyn Error>>;
    }
}

struct TestableProviderFactory {
    pub provider_expect: fn(Box<MockProvider>) -> Box<MockProvider>,
}

impl ProviderFactory for TestableProviderFactory {
    fn make_provider(&self, _: &WeatherProvider) -> Box<dyn Provider> {
        let provider_mock = Box::new(MockProvider::new());
        (self.provider_expect)(provider_mock)
    }
}

struct TestableDependencyFactory {
    pub data_storage_expect: fn(Box<MockDataStorage>) -> Box<MockDataStorage>,
    pub provider_expect: fn(Box<MockProvider>) -> Box<MockProvider>,
}

impl DependencyFactory for TestableDependencyFactory {
    fn make_data_storage(&self) -> Box<dyn DataStorage> {
        let data_storage_mock = Box::new(MockDataStorage::new());
        (self.data_storage_expect)(data_storage_mock)
    }

    fn make_provider_factory(&self) -> Box<dyn ProviderFactory> {
        Box::new(TestableProviderFactory {
            provider_expect: self.provider_expect,
        })
    }
}

#[test]
fn add_provider() {
    let dependency_factory = Box::new(TestableDependencyFactory {
        data_storage_expect: |mut data_storage_mock| {
            data_storage_mock
                .expect_get_provider()
                .times(1)
                .returning(|_| Err(static_err("err")));

            data_storage_mock
                .expect_add_provider()
                .times(1)
                .returning(|_| Ok(()));

            data_storage_mock
        },
        provider_expect: |mut provider_mock| {
            provider_mock
                .expect_get_weather()
                .times(1)
                .returning(|_, _| {
                    Ok(WeatherInfo {
                        description: None,
                        temperature: None,
                        humidity: None,
                        pressure: None,
                    })
                });
            provider_mock
        },
    });

    let processor = Processor::new(dependency_factory);

    let add_provider_command1 = Cli {
        command: WeatherCommand::Provider(ProviderCommand {
            command: ProviderSubcommand::Add(WeatherProvider {
                name: "name".to_string(),
                provider: arguments::Provider::AccuWeather,
                api_key: "api_key".to_string(),
            }),
        }),
    };

    assert_eq!(Some(()), processor.run(add_provider_command1).ok());
}

#[test]
fn add_existing_provider() {
    let dependency_factory = Box::new(TestableDependencyFactory {
        data_storage_expect: |mut data_storage_mock| {
            data_storage_mock
                .expect_get_provider()
                .times(1)
                .returning(|_| {
                    Ok(WeatherProvider {
                        name: "name".to_string(),
                        provider: arguments::Provider::AccuWeather,
                        api_key: "api_key".to_string(),
                    })
                });

            data_storage_mock.expect_add_provider().times(0);

            data_storage_mock
        },
        provider_expect: |mut provider_mock| {
            provider_mock
                .expect_get_weather()
                .times(1)
                .returning(|_, _| {
                    Ok(WeatherInfo {
                        description: None,
                        temperature: None,
                        humidity: None,
                        pressure: None,
                    })
                });
            provider_mock
        },
    });

    let processor = Processor::new(dependency_factory);

    let add_provider_command1 = Cli {
        command: WeatherCommand::Provider(ProviderCommand {
            command: ProviderSubcommand::Add(WeatherProvider {
                name: "name".to_string(),
                provider: arguments::Provider::AccuWeather,
                api_key: "api_key".to_string(),
            }),
        }),
    };

    assert_eq!(false, processor.run(add_provider_command1).is_ok());
}
