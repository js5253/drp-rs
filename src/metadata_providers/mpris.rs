#[cfg(target_os = "linux")]
use mpris::{self, PlayerFinder};
#[cfg(target_os = "linux")]

use crate::metadata_providers::MediaType;
use super::MetadataProvider;
#[cfg(target_os = "linux")]
use super::MetadataProvider;

macro_rules! platform_implementation {
    () => {
        
    };
}

#[derive(Default)]
pub struct MprisParser {
}

#[cfg(not(target_os="linux"))] 
impl MetadataProvider for MprisParser {
    fn get_playing_metadata(&self) -> Option<super::Metadata> {
        None
    }
}

#[cfg(target_os="linux")]
impl MetadataProvider for MprisParser {
    fn get_playing_metadata(&self) -> Option<MetadataProvider> {
        let player = PlayerFinder::new().unwrap().find_first();
        
        match player {
            Ok(player) => {
                let status = player.get_playback_status().unwrap();
                let progress = player.get_position().unwrap();
                let metadata = player.get_metadata().unwrap();
                let metadata = metadata.as_hashmap();
                // println!("{:?}", metadata);
    
                Some(MetadataProvider {
                    title: format!("{} - {}", metadata.get("xesam:title").unwrap().as_str().unwrap().to_string(), metadata.get("xesam:artist").unwrap().as_str_array().unwrap().join(", ")),
                    aux_title: Some(metadata.get("xesam:album").unwrap().as_str().unwrap().to_string()),
                    playback_state: None,
                    progress: Some(progress),
                    state: None,
                    providerName: String::from("MPRIS provider"),
                    subproviderName: Some(player.identity().to_string()),
                    mediaType: MediaType::MIXED,
                    metadata_media: Some(metadata.get("mpris:artUrl").unwrap().as_str().unwrap().to_string()),
    
                })
            },
            Err(_) => None,
        }
    }
}
