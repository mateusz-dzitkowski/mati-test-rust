use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub database_url: String,
    pub pool_max_connections: u32,
    pub app_port: u16,
}

pub fn get_config() -> Result<Config, config::ConfigError> {
    let env = config::Environment::default();

    let conf = config::Config::builder()
        .add_source(env)
        .build()?;
    conf.try_deserialize()
}
