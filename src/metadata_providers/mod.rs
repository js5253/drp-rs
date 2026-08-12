use std::time::Duration;

#[cfg(target_os = "linux")]
use mpris::PlaybackStatus;
// use windows::Media::{MediaPlaybackType, Control::GlobalSystemMediaTransportControlsSessionPlaybackStatus};
#[cfg(target_os = "windows")]
use ::windows::Media::{
    Control::GlobalSystemMediaTransportControlsSessionPlaybackStatus, MediaPlaybackType,
};

use crate::settings::AppSettings;

#[derive(Debug, PartialEq, Eq)]
pub enum PlaybackState {
    PLAYING,
    PAUSED,
    STOPPED,
    UNKNOWN,

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


#[derive(Debug, PartialEq, Eq)]
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
#[derive(Debug, PartialEq)]
pub struct Metadata {
    pub title: String,
    pub aux_title: Option<String>,
    /*Progress in percentage of 1 */
    pub progress: Option<Duration>,
    pub state: Option<PlaybackState>,
    pub provider_name: String,
    pub subprovider_name: Option<String>,
    pub media_type: MediaType,
    pub metadata_media: Option<String>,
}

pub trait MetadataProvider {
    fn get_playing_metadata(&self, settings: &AppSettings) -> Option<Metadata>;
}

pub mod extension;
pub mod mpris_parser;
pub mod windows;
