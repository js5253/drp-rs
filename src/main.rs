use std::{thread, time::Duration};

use reqwest::{
    blocking::{get, Client},
    cookie::Jar,
};
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
fn get_playing_metadata_tautulli(config: &Config) -> Option<TautulliSession> {
    let client = Client::new();
    //let jar = Jar::default();

    //jar.add_cookie_str(APP_CONFIG.tautulli_server_cookie, APP_CONFIG.tautulli_server_url);

    let data = client
        .get(format!("{}get_activity", &config.get_string("tautulli_server_url").unwrap()))
        .header("Cookie", &config.get_string("tautulli_server_cookie").unwrap())
        .send();

    let d = data.unwrap().json::<TautulliResponse>();

    let binding = d.unwrap();
    let user_session: Vec<&TautulliSession> = binding
        .sessions
        .iter()
        .filter(|session| session.user == config.get_string("username").unwrap())
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
    let config = Config::builder().add_source(config::File::with_name("App.toml")).build().unwrap();
    let prev_playing = get_playing_metadata_tautulli(&config);
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
                        .state(&format!("► {} - {}", full_title, state))
                )
                .unwrap();
    };
    loop {
        let playing: Option<TautulliSession> = get_playing_metadata_tautulli(&config);
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
                        .state(&format!("{} {} - {}", get_playback_sign(&state), full_title, state))
                        .details(&(media_type + " - " + &(percent.to_string() + "% played")))
                        .buttons(vec![Button::new("View Details", &(String::from("https://www.justwatch.com/us/search?q=") + &encode(&full_title)))])
                )
                .unwrap();
        }
        thread::sleep(Duration::from_secs(3))
    }
}
