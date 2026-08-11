use crate::metadata_providers::MediaType;

#[cfg(target_os = "linux")]
use crate::metadata_providers::Metadata;
use crate::{metadata_providers::MetadataProvider, settings::AppSettings};

#[cfg(target_os = "linux")]
use mpris::{self, PlayerFinder};
use serde::Deserialize;

#[derive(Deserialize)]
struct ProcessTypes {
    music: Vec<String>,
    video: Vec<String>,
}

pub struct MprisParser {
    process_types: ProcessTypes,
}

#[cfg(not(target_os = "linux"))]
impl MetadataProvider for MprisParser {
    fn get_playing_metadata(&self, settings: &AppSettings) -> Option<super::Metadata> {
        None
    }
}

impl MprisParser {
    pub fn new() -> Self {
        Self {
            process_types: toml::from_str(include_str!("../processes.toml")).unwrap_or(ProcessTypes {music: Vec::new(), video: Vec::new()})
        }
    }
}

#[cfg(target_os = "linux")]
impl MetadataProvider for MprisParser {
    fn get_playing_metadata(&self, settings: &AppSettings) -> Option<Metadata> {
        let player = PlayerFinder::new().ok()?;
        let player = player.find_active().ok()?;

        let _status = player.get_playback_status().ok()?;
        let progress = player.get_position().ok()?;
        let binding = player.get_metadata().ok()?;
        let metadata = binding.as_hashmap();
        let media_type: MediaType = {
            if self
                .process_types
                .music
                .contains(&player.identity().to_string())
            {
                MediaType::AUDIO
            } else if self
                .process_types
                .video
                .contains(&player.identity().to_string())
            {
                MediaType::VIDEO
            } else {
                MediaType::MIXED
            }
        };
        // println!("{:?}", metadata);

        Some(Metadata {
            title: format!(
                "{} - {}",
                metadata.get("xesam:title")?.as_str()?,
                metadata.get("xesam:artist")?.as_str_array()?.join(", ")
            ),
            aux_title: Some(metadata.get("xesam:album")?.as_str()?.to_string()),
            progress: Some(progress),
            state: None,
            provider_name: String::from("MPRIS provider"),
            subprovider_name: Some(player.identity().to_string()),
            media_type,
            metadata_media: Some(metadata.get("mpris:artUrl")?.as_str()?.to_string()),
        })
    }
}
