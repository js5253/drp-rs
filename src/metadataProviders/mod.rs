use std::time::Duration;


#[derive(Debug, PartialEq, Eq)]
pub enum PlaybackState {
    PLAYING,
    PAUSED,

}
#[derive(Debug, PartialEq, Eq)]
pub enum MediaType {
    AUDIO,
    VIDEO,
    MIXED
}
#[derive(Debug, PartialEq)]
pub struct MetadataProvider {
    pub title: String,
    pub aux_title: Option<String>,
    pub playback_state: Option<String>,
    /*Progress in percentage of 1 */
    pub progress: Option<Duration>,
    pub state: Option<PlaybackState>,
    pub providerName: String,
    pub subproviderName: Option<String>,
    pub mediaType: MediaType,
    pub metadata_media: Option<String>


}

pub mod tautulli;
pub mod mpris;
