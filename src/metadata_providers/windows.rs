use super::MetadataProvider;
#[derive(Default)]
pub struct WindowsParser {}
#[cfg(not(target_os="windows"))] 
impl MetadataProvider for WindowsParser {
    fn get_playing_metadata(&self) -> Option<super::Metadata> {
        None
    }
}
#[cfg(target_os="windows")]
impl MetadataProvider for WindowsParser {
    fn get_playing_metadata(&self) -> Option<super::Metadata> {
        None
    }
}