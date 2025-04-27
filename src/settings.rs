use serde::Deserialize;
use secrecy::SecretString;

#[derive(Deserialize)]
pub struct Settings {
    pub database_url: SecretString,
    pub pool_max_connections: u32,
    pub app_port: u16,
}

pub fn get_settings() -> Result<Settings, config::ConfigError> {
    let env = config::Environment::default();

    let conf = config::Config::builder()
        .add_source(env)
        .build()?;
    conf.try_deserialize()
}
