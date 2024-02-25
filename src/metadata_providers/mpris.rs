use crate::{metadata_providers::MetadataProvider, settings::AppSettings};

#[cfg(target_os = "linux")]
use mpris::{self, PlayerFinder};


#[derive(Default)]
pub struct MprisParser {
}

#[cfg(not(target_os="linux"))] 
impl MetadataProvider for MprisParser {
    fn get_playing_metadata(&self, settings: &AppSettings) -> Option<super::Metadata> {
        None
    }
}

#[cfg(target_os="linux")]
impl MetadataProvider for MprisParser {
    fn get_playing_metadata(&self) -> Option<Metadata> {
        let player = PlayerFinder::new().unwrap().find_first();
        
        match player {
            Ok(player) => {
                let _status = player.get_playback_status().unwrap();
                let progress = player.get_position().unwrap();
                let metadata = player.get_metadata().unwrap();
                let metadata = metadata.as_hashmap();
                // println!("{:?}", metadata);
    
                Some(Metadata {
                    title: format!("{} - {}", metadata.get("xesam:title").unwrap().as_str().unwrap().to_string(), metadata.get("xesam:artist").unwrap().as_str_array().unwrap().join(", ")),
                    aux_title: Some(metadata.get("xesam:album").unwrap().as_str().unwrap().to_string()),
                    progress: Some(progress),
                    state: None,
                    provider_name: String::from("MPRIS provider"),
                    subprovider_name: Some(player.identity().to_string()),
                    media_type: MediaType::MIXED,
                    metadata_media: Some(metadata.get("mpris:artUrl").unwrap().as_str().unwrap().to_string()),
    
                })
            },
            Err(_) => None,
        }
    }
}
