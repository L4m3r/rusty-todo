use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub database: DatabaseSettings,
    pub port: i32,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: i32,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }

    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}

pub fn get_config() -> Result<Config, config::ConfigError> {
    let cfg = config::Config::builder()
        .add_source(config::File::with_name("./config/config.yaml"))
        .build()?;
    cfg.try_deserialize::<Config>()
}
