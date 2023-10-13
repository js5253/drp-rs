use std::{thread, time::Duration};

use reqwest::blocking::Client;
use config::Config;
fn get_playback_sign(status: &str) -> &str {
    match status {
        "paused" => "⏸︎",
        "playing" => "⏵︎",
        _ => ""
    }
}
struct PlayingMetadata {
    name: String,
    artist: String,
    percentage: Option<u8>,
}
use discord_rich_presence::{
    activity::{self, Activity, Timestamps, Button},
    DiscordIpc, DiscordIpcClient,
};
use serde::Deserialize;
use serde_aux::prelude::deserialize_number_from_string;
use urlencoding::encode;

struct AppConfig {
    tautulli_server_url: String,
    tautulli_server_cookie: String,
    username: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
struct TautulliSession {
    user: String,
    full_title: String,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    progress_percent: u8,
    state: String,
    media_type: String
}

#[derive(Deserialize, Debug)]
struct TautulliResponse {
    sessions: Vec<TautulliSession>,
}
fn get_playing_metadata_tautulli(config: &AppConfig) -> Option<TautulliSession> {
    let client = Client::new();
    //let jar = Jar::default();

    //jar.add_cookie_str(APP_CONFIG.tautulli_server_cookie, APP_CONFIG.tautulli_server_url);

    let data = client
        .get(format!("{}get_activity", &config.tautulli_server_url))
        .header("Cookie", &config.tautulli_server_cookie)
        .send();

    let d = data.unwrap().json::<TautulliResponse>();

    let binding = d.unwrap();
    let user_session: Vec<&TautulliSession> = binding
        .sessions
        .iter()
        .filter(|session| session.user == config.username)
        .collect();
    match user_session.len() {
        0 => return None,
        _ => return Some(user_session[0].clone()),
    };
}

// fn get_playing_metadata(player: &Option<Player>) -> Option<PlayingMetadata> {
//     match player {
//         Ok(player) => {
//             let metadata = player.get_metadata().unwrap().as_hashmap();

//             let title = metadata.get("fs:title").unwrap().as_str();
//             let artist = metadata.get("fs:artist").unwrap().as_str_array()[0];

//             let playing = PlayingMetadata { name, artist, percentage: None };

//             Some(playing)
//         }
//         Err(_) => None, // can't find any player
//     }
// }

fn main() {
    let app_config: AppConfig = AppConfig {
        tautulli_server_url: "http://server1-stats.omniplex.club/".to_string(), //get_activity
        tautulli_server_cookie: "tautulli_token_608ecf9fab56436b96d62243b0a05470=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyX2lkIjoxMTIxOTUyMCwidXNlciI6IlByb2JhYmx5QUhhY2hlciIsInVzZXJfZ3JvdXAiOiJndWVzdCIsImV4cCI6MTY5OTc0MTE5OH0.dxRpnqegZkgCr57k068-Km5CcTOxM9A-JQ9QF4zfZW0".to_string(),
        username: "ProbablyAHacher".to_string()
    };
    let prev_playing = get_playing_metadata_tautulli(&app_config);
    let mut ipc_client = DiscordIpcClient::new("1162169068418248764").unwrap();
    ipc_client.connect().unwrap();

    println!("Discord Playing Thing");
    if prev_playing != None {
        let p = prev_playing.clone().unwrap();
        let full_title = p.full_title;
        let state = p.state;

        let _ = ipc_client
                .set_activity(
                    Activity::new()
                        .state(&format!("{} {} - {}", get_playback_sign(&state), full_title, state))
                        // .details(&(media_type + " - " + &(percent.to_string() + "% played")))

                )
                .unwrap();
    };
    loop {
        let playing: Option<TautulliSession> = get_playing_metadata_tautulli(&app_config);
        if playing == None {
            let _ = ipc_client.clear_activity();
        }
        if playing != prev_playing && playing != None {
            let playing = playing.unwrap();
            let full_title = playing.full_title;
            let state = playing.state;
            let percent = playing.progress_percent;
            let media_type = playing.media_type;
            let _ = ipc_client
                .set_activity(
                    Activity::new()
                    .state(&(media_type + " - " + &(percent.to_string() + "% played")))
                        .details(&format!("{} {} - {}", get_playback_sign(&state), full_title, state))
                        .buttons(vec![
                            Button::new("View Details", &(String::from("https://www.justwatch.com/us/search?q=") + &encode(&full_title))),
                            //Button::new("View on Plex", &(String::from("https://app.plex.tv/desktop/#!/search?q=") + &encode(&full_title)))
                            ])
                )
                .unwrap();
        }
        thread::sleep(Duration::from_secs(4))
    }
}
