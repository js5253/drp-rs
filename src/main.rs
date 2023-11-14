use std::{thread, time::Duration};

use config::{Config, ConfigError};

mod metadata_providers;
use metadata_providers::{mpris::MprisParser, windows::WindowsParser, MetadataProvider};

fn get_playback_sign(status: &str) -> &str {
    match status {
        "paused" => "⏸︎",
        "playing" => "⏵︎",
        _ => "",
    }
}

use discord_rich_presence::{
    activity::{Activity, Assets},
    DiscordIpc, DiscordIpcClient,
};
use lazy_static::lazy_static;
use serde::Deserialize;

use crate::metadata_providers::extension;

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

#[derive(Deserialize, Debug)]
pub struct AppSettings {
    discord_username: String,
    tautulli_server_url: String,
    tautulli_server_cookie: String,
    parsers: Vec<String>,
}
impl AppSettings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(config::File::with_name("App"))
            .build()
            .unwrap();

        s.try_deserialize()
    }
}

fn get_default_provider() -> Option<Box<dyn MetadataProvider>> {
    // in the meantime, use only the first metadata provider.
    if cfg!(linux) {
        Some(Box::new(MprisParser {}))
    } else if cfg!(windows) {
        Some(Box::new(WindowsParser {}))
    } else {
        return None;
    }
}

fn pretty_time(dur: Duration) -> String {
    let seconds = dur.as_secs();
    let minutes = seconds / 60;
    let remaining_seconds = seconds % 60;
    format!("{:02}:{:02}", minutes, remaining_seconds)
}

lazy_static! {
    pub static ref SETTINGS: AppSettings = AppSettings::new().expect("Config file is incorrect.");
}

// #[tokio::main]
// async fn main() {
//     extension::main();
//     thread::spawn(|| {

//     }).join();

#[tokio::main]
async fn main() {
    println!("{:?}", SETTINGS.parsers);

    if SETTINGS.parsers.contains(&"extension".to_string()) {
        tokio::spawn(extension::main());
        // extension::main().await;
    }
    loop {
        let mut ipc_client: DiscordIpcClient =
            DiscordIpcClient::new("1162169068418248764").expect("Could not connect to Discord");
        ipc_client.connect().unwrap();
        let mut time_elapsed: u64 = 0;

        println!("Something here...");
        let prev_playing = get_default_provider();

        match prev_playing {
            Some(prev_playing) => {
                println!("Discord Playing Thing");
                loop {
                    if let Some(playing_metadata) = prev_playing.get_playing_metadata() {
                        let _ = ipc_client
                            .set_activity(
                                Activity::new()
                                    .assets(
                                        Assets::new()
                                            .large_image(
                                                "https://cdn.frankerfacez.com/emoticon/660211/4",
                                            )
                                            .small_image(
                                                "https://cdn.frankerfacez.com/emoticon/660211/4",
                                            ),
                                    )
                                    .details(&format!(
                                        "{} - {}",
                                        &playing_metadata.title,
                                        &playing_metadata.aux_title.unwrap_or_default()
                                    ))
                                    // .assets(Assets::new().large_image(&curr_playing.metadata_media.clone().unwrap()))
                                    .state(
                                        format!(
                                            "{} played",
                                            pretty_time(
                                                playing_metadata
                                                    .progress
                                                    .unwrap_or(Duration::from_secs(time_elapsed))
                                            )
                                        )
                                        .as_str(),
                                    ),
                            )
                            .unwrap();
                    };
                    time_elapsed += 3;
                    thread::sleep(Duration::from_secs(3));
                }
            }
            None => {}
        }
    }
}
