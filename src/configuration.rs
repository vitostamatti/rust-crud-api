use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct ApplicationConfiguration {
    pub port: u16,
    pub host: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct DatabaseConfiguration {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Configuration {
    pub app: ApplicationConfiguration,
    pub db: DatabaseConfiguration,
}

pub fn get_configuration() -> Result<Configuration, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine current directory");

    let configuration_directory = base_path.join("configuration");

    let configuration = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("configuration.yaml"),
        ))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()
        .expect("Failed to load configuration.");

    configuration.try_deserialize::<Configuration>()
}
