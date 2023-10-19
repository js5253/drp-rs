use std::{thread, time::Duration};

use config::Config;
use reqwest::blocking::Client;

mod metadataProviders;
use metadataProviders::{mpris, tautulli, MetadataProvider};

fn get_playback_sign(status: &str) -> &str {
    match status {
        "paused" => "⏸︎",
        "playing" => "⏵︎",
        _ => "",
    }
}
struct PlayingMetadata {
    name: String,
    artist: String,
    percentage: Option<u8>,
}
use discord_rich_presence::{
    activity::{self, Activity, Button, Timestamps, Assets},
    DiscordIpc, DiscordIpcClient,
};
use lazy_static::lazy_static;
use serde::Deserialize;
use urlencoding::encode;
use crate::metadataProviders::tautulli::TautulliSession;

pub struct AppConfig {
    tautulli_server_url: String,
    tautulli_server_cookie: String,
    username: String,
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
fn get_all_providers() -> Vec<MetadataProvider> {
    // in the meantime, use only the first metadata provider.
    let mut providers = vec![mpris::get_playing_metadata().unwrap()];

    providers
}

fn pretty_time(seconds: u32) -> String {
    let minutes = seconds / 60;
    let remaining_seconds = seconds % 60;
    format!("{:02}:{:02}", minutes, remaining_seconds)
}


lazy_static! {
    pub static ref SETTINGS: Config = Config::builder().add_source(config::File::with_name("App")).build().unwrap();

}

fn main() {
    let prev_playing = &get_all_providers()[0];
    println!("Settings loaded: {}", SETTINGS.get_string("username").unwrap());
    // let app_config: AppConfig = AppConfig {
    //     tautulli_server_url: "http://server1-stats.omniplex.club/".to_string(), //get_activity
    //     tautulli_server_cookie: "tautulli_token_608ecf9fab56436b96d62243b0a05470=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyX2lkIjoxMTIxOTUyMCwidXNlciI6IlByb2JhYmx5QUhhY2hlciIsInVzZXJfZ3JvdXAiOiJndWVzdCIsImV4cCI6MTY5OTc0MTE5OH0.dxRpnqegZkgCr57k068-Km5CcTOxM9A-JQ9QF4zfZW0".to_string(),
    //     username: "ProbablyAHacher".to_string()
    // };
    // let prev_playing = get_playing_metadata_tautulli(&app_config);
    let mut ipc_client = DiscordIpcClient::new("1162169068418248764").unwrap();
    ipc_client.connect().unwrap();
    let mut time_elapsed: u32 = 0;

    let _ = ipc_client.set_activity(Activity::new()
    .details(&format!("{} - {}", prev_playing.title, prev_playing.aux_title.clone().unwrap()))
    // .assets(Assets::new().large_image(&prev_playing.metadata_media.clone().unwrap()))
    .state(format!("{} played - {}", pretty_time(time_elapsed), &prev_playing.subproviderName.clone().unwrap()).as_str()))
    .unwrap();

    println!("Discord Playing Thing");
    loop {
        let curr_playing = &get_all_providers()[0];
        if prev_playing != curr_playing {
            time_elapsed = 0;
        }
            let _ = ipc_client.set_activity(Activity::new()
            .details(&format!("{} - {}", curr_playing.title, curr_playing.aux_title.clone().unwrap()))
            // .assets(Assets::new().large_image(&curr_playing.metadata_media.clone().unwrap()))
            .state(format!("{} played - {}", pretty_time(time_elapsed), &curr_playing.subproviderName.clone().unwrap()).as_str()))
            .unwrap();
        //}
        time_elapsed += 3;
        thread::sleep(Duration::from_secs(3))
    }
    // if prev_playing != None {
    //     let p = prev_playing.clone().unwrap();
    //     let full_title = p.full_title;
    //     let state = p.state;

    //     let _ = ipc_client
    //             .set_activity(
    //                 Activity::new()
    //                     .state(&format!("{} {} - {}", get_playback_sign(&state), full_title, state))
    //                     // .details(&(media_type + " - " + &(percent.to_string() + "% played")))

    //             )
    //             .unwrap();
    // };
    // loop {
    //     let playing: Option<TautulliSession> = get_playing_metadata_tautulli(&app_config);
    //     if playing == None {
    //         let _ = ipc_client.clear_activity();
    //     }
    //     if playing != prev_playing && playing != None {
    //         let playing = playing.unwrap();
    //         let full_title = playing.full_title;
    //         let state = playing.state;
    //         let percent = playing.progress_percent;
    //         let media_type = playing.media_type;
    //         let _ = ipc_client
    //             .set_activity(
    //                 Activity::new()
    //                 .state(&(media_type + " - " + &(percent.to_string() + "% played")))
    //                     .details(&format!("{} {} - {}", get_playback_sign(&state), full_title, state))
    //                     .buttons(vec![
    //                         Button::new("View Details", &(String::from("https://www.justwatch.com/us/search?q=") + &encode(&full_title))),
    //                         //Button::new("View on Plex", &(String::from("https://app.plex.tv/desktop/#!/search?q=") + &encode(&full_title)))
    //                         ])
    //             )
    //             .unwrap();
    //     }
    //     thread::sleep(Duration::from_secs(4))
    // }
}
