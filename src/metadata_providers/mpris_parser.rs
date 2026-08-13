#[cfg(target_os = "linux")]
use std::{ops::Deref, time::Duration};

use crate::metadata_providers::MediaType;

#[cfg(target_os = "linux")]
use crate::metadata_providers::{Metadata, PlaybackState};
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
            process_types: toml::from_str(include_str!("../processes.toml")).unwrap_or(
                ProcessTypes {
                    music: Vec::new(),
                    video: Vec::new(),
                },
            ),
        }
    }
}

#[cfg(target_os = "linux")]
impl MetadataProvider for MprisParser {
    fn get_playing_metadata(&self, settings: &AppSettings) -> Option<Metadata> {
        let player = PlayerFinder::new().ok()?;
        let player = player.find_active().ok()?;

        let status = player.get_playback_status().ok();
        let progress = player.get_position().ok().filter(|pred| !pred.is_zero());
        let binding = player.get_metadata().ok()?;
        let metadata = binding.as_hashmap();
        println!("Hello World");
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
        let title: Option<&String> = metadata.get("xesam:title").and_then(|title| title.as_string());
        let artist: Option<String> = metadata.get("xesam:artist").and_then(|artist| artist.as_str_array()).map(|artist| artist.join(", ")); // could be None, in which case it should still go through the rest of the code and just show the title
        let aux_title = metadata.get("xesam:album").and_then(|aux_title| aux_title.as_string());
        // this is way too much of a mess; i'll check it out later
        let photo_url = metadata.get("mpris:artUrl").and_then(|url| url.as_string());
        let title = title?;
        let formatted_title = match artist {
            None => title.clone(),
            Some(artist) => format!("{} - {}", title, artist),
        };
        Some(Metadata {
            title: formatted_title.to_owned(),
            aux_title: aux_title.cloned(),
            progress,
            state: status.map(|status| {status.into()}),
            provider_name: String::from("MPRIS provider"),
            subprovider_name: Some(player.identity().to_string()),
            media_type,
            metadata_media: photo_url.cloned()
        })
    }
}
