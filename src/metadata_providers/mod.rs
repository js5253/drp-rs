use std::time::Duration;

use ::windows::Media::MediaPlaybackType;


#[derive(Debug, PartialEq, Eq)]
pub enum PlaybackState {
    PLAYING,
    PAUSED,

}
#[derive(Debug, PartialEq, Eq)]
pub enum MediaType {
    AUDIO,
    VIDEO,
    MIXED,
    UNKNOWN
}
#[cfg(target_os="windows")]
impl From<MediaPlaybackType> for MediaType {
    fn from(value: MediaPlaybackType) -> Self {
        match value {
            MediaPlaybackType(1) => MediaType::VIDEO,
            MediaPlaybackType(2) => MediaType::AUDIO,
            _ => MediaType::UNKNOWN
        }

    }
}
#[derive(Debug, PartialEq)]
pub struct Metadata {
    pub title: String,
    pub aux_title: Option<String>,
    pub playback_state: Option<String>,
    /*Progress in percentage of 1 */
    pub progress: Option<Duration>,
    pub state: Option<PlaybackState>,
    pub provider_name: String,
    pub subprovider_name: Option<String>,
    pub media_type: MediaType,
    pub metadata_media: Option<String>
}
pub trait MetadataProvider {
    fn get_playing_metadata(&self) -> Option<Metadata>;
}

pub mod tautulli;
pub mod mpris;  
pub mod windows;
pub mod mac;