use crate::settings::AppSettings;

use super::{Metadata, MetadataProvider};

struct TidalMetadataProvider;

impl MetadataProvider for TidalMetadataProvider {
    fn get_playing_metadata(&self, settings: &AppSettings) -> Option<Metadata> {
        println!("Using Tidal Metadata");
        None
    }
}