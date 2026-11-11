use std::{fmt, time::Duration};

use anyhow::anyhow;
#[cfg(target_os = "linux")]
use mpris::PlaybackStatus;
use serde::Deserialize;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
// use windows::Media::{MediaPlaybackType, Control::GlobalSystemMediaTransportControlsSessionPlaybackStatus};
#[cfg(target_os = "windows")]
use ::windows::Media::{
    Control::GlobalSystemMediaTransportControlsSessionPlaybackStatus, MediaPlaybackType,
};

use crate::settings::AppSettings;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PlaybackState {
    PLAYING,
    PAUSED,
    STOPPED,
    UNKNOWN,
}
impl fmt::Display for PlaybackState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlaybackState::PLAYING => write!(f, "Playing"),
            PlaybackState::PAUSED => write!(f, "Paused"),
            PlaybackState::STOPPED => write!(f, "Stopped"),
            PlaybackState::UNKNOWN => write!(f, ""),
        }
    }
}
#[cfg(target_os = "windows")]
impl From<GlobalSystemMediaTransportControlsSessionPlaybackStatus> for PlaybackState {
    fn from(value: GlobalSystemMediaTransportControlsSessionPlaybackStatus) -> Self {
        match value {
            GlobalSystemMediaTransportControlsSessionPlaybackStatus(4) => Self::PLAYING,
            GlobalSystemMediaTransportControlsSessionPlaybackStatus(5) => Self::PAUSED,
            _ => Self::UNKNOWN,
        }
    }
}

#[cfg(target_os = "linux")]
impl From<PlaybackStatus> for PlaybackState {
    fn from(value: PlaybackStatus) -> Self {
        match value {
            PlaybackStatus::Playing => PlaybackState::PLAYING,
            PlaybackStatus::Paused => PlaybackState::PAUSED,
            PlaybackStatus::Stopped => PlaybackState::STOPPED,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub enum MediaType {
    AUDIO,
    VIDEO,
    MIXED,
}
#[cfg(target_os = "windows")]
impl From<MediaPlaybackType> for MediaType {
    fn from(value: MediaPlaybackType) -> Self {
        match value {
            MediaPlaybackType(1) => MediaType::VIDEO,
            MediaPlaybackType(2) => MediaType::AUDIO,
            _ => MediaType::MIXED,
        }
    }
}
impl TryFrom<&str> for MediaType {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "audio" => Ok(MediaType::AUDIO),
            "video" => Ok(MediaType::VIDEO),
            "mixed" => Ok(MediaType::MIXED),
            _ => Err(anyhow!(""))
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct Metadata {
    pub title: String,
    pub aux_title: Option<String>,
    pub progress: Option<Duration>,
    pub state: Option<PlaybackState>,
    pub provider_name: String,
    pub subprovider_name: Option<String>,
    pub media_type: MediaType,
    pub metadata_media: Option<String>,
}

pub trait MetadataProvider {
    fn register_thread(&self, tx: mpsc::Sender<Metadata>, token: CancellationToken);
}

pub mod extension;
pub mod mpris_parser;
pub mod windows;
