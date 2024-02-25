use super::{Metadata, MetadataProvider};

struct TidalMetadataProvider;

impl MetadataProvider for TidalMetadataProvider {
    fn get_playing_metadata(&self) -> Option<Metadata> {
        println!("Using Tidal Metadata");
        None
    }
}