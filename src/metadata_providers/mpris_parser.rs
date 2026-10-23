#[cfg(target_os = "linux")]
use std::{ops::Deref, pin::Pin, time::Duration};

use crate::metadata_providers::MediaType;

#[cfg(not(target_os = "linux"))]
use crate::metadata_providers::Metadata;
#[cfg(target_os = "linux")]
use crate::metadata_providers::{Metadata, PlaybackState};
use crate::{metadata_providers::MetadataProvider, settings::AppSettings};

use anyhow::Error;
use mpris::FindingError::{self, NoPlayerFound};
#[cfg(target_os = "linux")]
use mpris::{self, PlayerFinder};
use serde::Deserialize;
#[cfg(not(target_os = "linux"))]
use tokio::sync::mpsc;
#[cfg(target_os = "linux")]
use tokio::sync::mpsc;
use tokio::time::sleep;
#[cfg(target_os = "linux")]
use tokio_util::sync::CancellationToken;
#[cfg(not(target_os = "linux"))]
use tokio_util::sync::CancellationToken;

#[derive(Deserialize, Clone)]
struct ProcessTypes {
    music: Vec<String>,
    video: Vec<String>,
}

pub struct MprisParser {
    process_types: ProcessTypes,
}

#[cfg(not(target_os = "linux"))]
impl MetadataProvider for MprisParser {
    fn register_thread(&self, mut tx: mpsc::Sender<Metadata>, token: CancellationToken) {}
}

impl MprisParser {
    pub fn new() -> Self {
        Self {
            process_types: toml::from_str(include_str!("../../config/processes.toml")).unwrap_or(
                ProcessTypes {
                    music: Vec::new(),
                    video: Vec::new(),
                },
            ),
        }
    }
}
impl MprisParser {
    async fn thread(
        process_types: ProcessTypes,
        tx: mpsc::Sender<Metadata>,
        token: CancellationToken,
    ) -> anyhow::Result<()> {
        let finder = PlayerFinder::new()?;
        let mut player = finder.find_active();

        while player.is_err() {
            player = finder.find_active();
            sleep(Duration::from_millis(500)).await;
        }
        let Ok(player) = player else {return Ok(())};

        loop {
            if token.is_cancelled() {
                return Ok(())
            }
            let status = player.get_playback_status().ok();
            let progress = player.get_position().ok().filter(|pred| !pred.is_zero());
            let Ok(binding) = player.get_metadata() else {continue};
            let metadata = binding.as_hashmap();
            let media_type: MediaType = {
                    if process_types
                    .music
                    .contains(&player.identity().to_string())
                {
                    MediaType::AUDIO
                } else if process_types
                    .video
                    .contains(&player.identity().to_string())
                {
                    MediaType::VIDEO
                } else {
                    MediaType::MIXED
                }
            };
            let title = metadata
                .get("xesam:title")
                .and_then(|title| title.as_string());
            let artist = metadata
                .get("xesam:artist")
                .and_then(|artist| artist.as_str_array())
                .map(|artist| artist.join(", "));
            let aux_title = metadata
                .get("xesam:album")
                .and_then(|aux_title| aux_title.as_string())
                .filter(|pred| !pred.is_empty());
            let photo_url = metadata.get("mpris:artUrl").and_then(|url| url.as_string());
            let Some(title) = title else {continue};
            let formatted_title = match artist {
                None => title.clone(),
                Some(artist) => format!("{} - {}", title, artist),
            };
            let _ = tx.send(Metadata {
                title: formatted_title.to_owned(),
                aux_title: aux_title.cloned(),
                progress,
                state: status.map(|status| status.into()),
                provider_name: String::from("MPRIS provider"),
                subprovider_name: Some(player.identity().to_string()),
                media_type,
                metadata_media: photo_url.cloned(),
            }).await;
        }
    }
}

#[cfg(target_os = "linux")]
impl MetadataProvider for MprisParser {
    fn register_thread(&self, tx: mpsc::Sender<Metadata>, token: CancellationToken) {
        let process_types = self.process_types.clone();
        // tokio::spawn(async move { 
        //     let _ = Self::thread(process_types, tx, token).await;
        // });
    }
}
