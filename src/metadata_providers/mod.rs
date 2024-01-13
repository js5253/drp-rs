use std::time::Duration;

// use windows::Media::{MediaPlaybackType, Control::GlobalSystemMediaTransportControlsSessionPlaybackStatus};
use ::windows::Media::{MediaPlaybackType, Control::GlobalSystemMediaTransportControlsSessionPlaybackStatus};


#[derive(Debug, PartialEq, Eq)]
pub enum PlaybackState {
    PLAYING,
    PAUSED,
    UNKNOWN,
}
#[cfg(target_os = "windows")]
impl From<GlobalSystemMediaTransportControlsSessionPlaybackStatus> for PlaybackState {
    fn from(value: GlobalSystemMediaTransportControlsSessionPlaybackStatus) -> Self {
        match value {
            GlobalSystemMediaTransportControlsSessionPlaybackStatus(4) => PlaybackState::PLAYING,
            GlobalSystemMediaTransportControlsSessionPlaybackStatus(5) => Self::PAUSED,
            _ => PlaybackState::UNKNOWN,
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum MediaType {
    AUDIO,
    VIDEO,
    MIXED,
    UNKNOWN,
}
#[cfg(target_os = "windows")]
impl From<MediaPlaybackType> for MediaType {
    fn from(value: MediaPlaybackType) -> Self {
        match value {
            MediaPlaybackType(1) => MediaType::VIDEO,
            MediaPlaybackType(2) => MediaType::AUDIO,
            _ => MediaType::UNKNOWN,
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
    fn get_playing_metadata(&self) -> Option<Metadata>;
}
#[macro_export]
macro_rules! platform_implementation {
    ($platform:literal $($code:tt)*) => {
        {
        #[cfg(target_os=$platform)] {
            impl MetadataProvider for MprisParser {
            fn get_playing_metadata(&self) -> Option<super::Metadata> {
                $($code)*
            }
        }
        #[cfg(not(target_os=$platform))] {
            fn get_playing_metadata(&self) -> Option<super::Metadata> {
                None
            }
        }
        }
    };
}
}

pub mod mac;
pub mod mpris;
pub mod extension;
pub mod tautulli;
pub mod windows;
