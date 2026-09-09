use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct AppSettings {
    pub discord_username: Option<String>,
    pub tautulli_server_url: Option<String>,
    pub tautulli_server_cookie: Option<String>,
    pub metadata_sources: Vec<String>,
    pub extension_host_enabled: bool,
}
impl AppSettings {
    pub fn load_or_default() -> Result<AppSettings, anyhow::Error> {
        let path = Path::new("config.toml");
        if fs::exists(path).is_ok_and(|item| item) {
            let config = fs::read_to_string("config.toml")?;
            let config = toml::from_str(config.as_str())?;

            Ok(config)
        } else {
            let config = AppSettings {
                discord_username: None,
                tautulli_server_url: None,
                tautulli_server_cookie: None,
                metadata_sources: vec!["native_now_playing".to_string()],
                extension_host_enabled: true,
            };
            fs::write(path, toml::to_string(&config)?)?;

            Ok(config)
        }
    }
    pub fn update(&mut self, new_settings: AppSettings) -> Result<(), anyhow::Error> {
        *self = new_settings;
        fs::write(Path::new("config.toml"), toml::to_string(&self)?);
        Ok(())
    }
}
