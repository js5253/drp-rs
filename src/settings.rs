use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use tracing::instrument;

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct AppSettings {
    pub discord_username: Option<String>,
    pub tautulli_server_url: Option<String>,
    pub tautulli_server_cookie: Option<String>,
    pub metadata_sources: Vec<String>,
}
impl AppSettings {
    #[instrument]
    pub fn load_or_default() -> AppSettings {
        if fs::exists("config/config.toml").is_ok() {
            let config: AppSettings = fs::read_to_string("config.toml")
                .and_then(|config| Ok(toml::from_str(&config).map_err(|_| anyhow!("A"))))
                .map_err(|_| anyhow!("B"))
                .flatten()
                .unwrap_or(AppSettings {
                    discord_username: None,
                    tautulli_server_url: None,
                    tautulli_server_cookie: None,
                    metadata_sources: vec!["native_now_playing".to_string()],
                });

            config
        } else {
            let config = AppSettings {
                discord_username: None,
                tautulli_server_url: None,
                tautulli_server_cookie: None,
                metadata_sources: vec!["native_now_playing".to_string()],
            };
            let _ = toml::to_string(&config).map_err(|_| anyhow!("")).and_then(|config| fs::write(path, config).map_err(|_| anyhow!("B")));

            config
        }
    }
    pub fn update(&mut self, new_settings: AppSettings) -> Result<(), anyhow::Error> {
        *self = new_settings;
        let _ = fs::write(Path::new("config.toml"), toml::to_string(&self)?);
        Ok(())
    }
}
