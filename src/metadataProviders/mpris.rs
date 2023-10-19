use mpris::{self, PlayerFinder};

use crate::metadataProviders::MediaType;

use super::MetadataProvider;

pub fn get_playing_metadata() -> Option<MetadataProvider> {
    let player = PlayerFinder::new().unwrap().find_first();
    
    match player {
        Ok(player) => {
            let metadata = player.get_metadata().unwrap();
            let metadata = metadata.as_hashmap();
            println!("{:?}", metadata);

            Some(MetadataProvider {
                title: format!("{} - {}", metadata.get("xesam:title").unwrap().as_str().unwrap().to_string(), metadata.get("xesam:artist").unwrap().as_str_array().unwrap()[0]),
                aux_title: Some(metadata.get("xesam:album").unwrap().as_str().unwrap().to_string()),
                playback_state: None,
                progress: None,
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