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

fn pretty_time(dur: Duration) -> String {
    let seconds = dur.as_secs();
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
    let mut ipc_client = DiscordIpcClient::new("1162169068418248764").unwrap();
    ipc_client.connect().unwrap();
    let mut time_elapsed: u64 = 0;

    // let _ = ipc_client.set_activity(Activity::new()
    // .details(&format!("{} - {}", prev_playing.title, prev_playing.aux_title.clone().unwrap()))
    // // .assets(Assets::new().large_image(&prev_playing.metadata_media.clone().unwrap()))
    // .state(format!("{} played - {}", pretty_time(time_elapsed), &prev_playing.subproviderName.clone().unwrap()).as_str()))
    // .unwrap();

    println!("Discord Playing Thing");
    loop {
        let curr_playing = &get_all_providers()[0];
        if prev_playing != curr_playing {
            time_elapsed = 0;
        }
            let _ = ipc_client.set_activity(Activity::new()
            .assets(
            Assets::new().large_image("https://cdn.frankerfacez.com/emoticon/660211/4")
            .small_image("https://cdn.frankerfacez.com/emoticon/660211/4"))
            .details(&format!("{} - {}", curr_playing.title, curr_playing.aux_title.clone().unwrap()))
            // .assets(Assets::new().large_image(&curr_playing.metadata_media.clone().unwrap()))
            .state(format!("{} played - {}", pretty_time(curr_playing.progress.unwrap_or(Duration::from_secs(time_elapsed))), &curr_playing.subproviderName.clone().unwrap()).as_str()))
            .unwrap();
        //}
        time_elapsed += 3;
        thread::sleep(Duration::from_secs(3))
            }
        }
