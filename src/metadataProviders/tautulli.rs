use reqwest::blocking::Client;
use serde::{Deserialize};
use serde_aux::prelude::deserialize_number_from_string;

use crate::SETTINGS;
use super::{MetadataProvider, MediaType};

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct TautulliSession {
    pub user: String,
    pub full_title: String,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub progress_percent: u8,
    pub state: String,
    pub media_type: String
}

#[derive(Deserialize, Debug)]
struct TautulliResponse {
    sessions: Vec<TautulliSession>,
}
pub fn get_playing_metadata() -> Option<MetadataProvider> {
    let client = Client::new();
    //let jar = Jar::default();

    //jar.add_cookie_str(APP_CONFIG.tautulli_server_cookie, APP_CONFIG.tautulli_server_url);

    let data = client
        .get(format!("{}get_activity", SETTINGS.get_string("tautulli_server_url").unwrap()))

        .header("User-Agent", "drp-rs tautulli status 0.1")
        .header("Cookie", SETTINGS.get_string("tautulli_server_cookie").unwrap())
        .send();

    let d = data.unwrap().json::<TautulliResponse>();

    let binding = d.unwrap();
    let user_session: Vec<&TautulliSession> = binding
        .sessions
        .iter()
        .filter(|session| session.user == SETTINGS.get_string("username").unwrap())
        .collect();
    match user_session.len() {
        0 => return None,
        _ => {
            let session = user_session[0];
            return Some(MetadataProvider { 
            title: String::new(),
            aux_title: None,
            playback_state: None,
            progress: None,
            state: None,
            providerName: todo!(),
            subproviderName: todo!(),
            mediaType: MediaType::MIXED,
            metadata_media: None
            
        })
    }
    };
}