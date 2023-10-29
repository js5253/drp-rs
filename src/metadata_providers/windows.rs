use std::thread::current;

use super::{MetadataProvider, Metadata, MediaType};
use futures::executor;
use windows::Media::{self, Control::{GlobalSystemMediaTransportControlsSession, GlobalSystemMediaTransportControlsSessionManager}, MediaPlaybackType};

enum PlaybackType {
    UNKNOWN = 0,
    MUSIC = 1,
    VIDEO = 2
}

#[derive(Default)]
pub struct WindowsParser {}
#[cfg(not(target_os="windows"))] 
impl MetadataProvider for WindowsParser {
    fn get_playing_metadata(&self) -> Option<super::Metadata> {
        None
    }
}
#[cfg(target_os="windows")]
async fn get_playback_metadata() -> Option<super::Metadata> {
    let a = GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().await.unwrap();
    let current_session = a.GetCurrentSession().unwrap();
    let playback_info = current_session.TryGetMediaPropertiesAsync().unwrap().await.unwrap();
    
    let title = playback_info.Title().unwrap().to_string();
    let artist = playback_info.Artist().unwrap().to_string();
    let media_type = MediaType::from(playback_info.PlaybackType().unwrap().Value().unwrap());

    

    println!("{:?}", media_type);

    Some(super::Metadata {
        title,
        aux_title: Some(artist),
        playback_state: None,
        progress: None,
        state: None,
        provider_name: "Windows Media".to_string(),
        subprovider_name: None,
        media_type: media_type,
        metadata_media: None,
    })
}
#[cfg(target_os="windows")]
impl MetadataProvider for WindowsParser {
    fn get_playing_metadata(&self) -> Option<super::Metadata> {


        executor::block_on(get_playback_metadata())
    }
}