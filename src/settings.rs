use config::{Config, ConfigError};
use serde::{Deserialize, Serialize};
use std::fs;
use toml;

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct AppSettings {
    pub discord_username: String,
    pub tautulli_server_url: String,
    pub tautulli_server_cookie: String,
    pub metadata_sources: Vec<String>,
}
impl AppSettings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(config::File::with_name("App"))
            .build()
            .unwrap();

        s.try_deserialize()
    }
    pub fn write(&self) -> Result<Self, ConfigError> {
        let _config_file = fs::write(
            "App.toml",
            toml::to_string(&self).expect("Couldn't write settings back to file..."),
        );
        Ok(self.clone())
    }
}