use std::{thread, time::Duration};

use config::Config;

mod metadata_providers;
use metadata_providers::{mpris::MprisParser, MetadataProvider, windows::WindowsParser};

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
fn get_default_provider() -> Option<Box<dyn MetadataProvider>> {
// in the meantime, use only the first metadata provider.
if cfg!(linux) {
        Some(Box::new(MprisParser{}))
    } else if cfg!(windows) {
        Some(Box::new(WindowsParser{}))
    } else {
        return None
    }
}

fn pretty_time(dur: Duration) -> String {
    let seconds = dur.as_secs();
    let minutes = seconds / 60;
    let remaining_seconds = seconds % 60;
    format!("{:02}:{:02}", minutes, remaining_seconds)
}

lazy_static! {
    pub static ref SETTINGS: Config = Config::builder()
        .add_source(config::File::with_name("App"))
        .build()
        .unwrap();
}

fn main() {
    let prev_playing = get_default_provider().unwrap();
    let prev_playing = prev_playing.get_playing_metadata().unwrap();
    println!(
        "Settings loaded: {}",
        SETTINGS.get_string("username").unwrap()
    );
    let mut ipc_client = DiscordIpcClient::new("1162169068418248764").unwrap();
    ipc_client.connect().unwrap();
    let mut time_elapsed: u64 = 0;

    println!("Discord Playing Thing");
    loop {
        let curr_playing = get_default_provider().unwrap();
        let curr_playing = curr_playing.get_playing_metadata().unwrap();
        if prev_playing != curr_playing {
            time_elapsed = 0;
        }
        let _ = ipc_client
            .set_activity(
                Activity::new()
                    .assets(
                        Assets::new()
                            .large_image("https://cdn.frankerfacez.com/emoticon/660211/4")
                            .small_image("https://cdn.frankerfacez.com/emoticon/660211/4"),
                    )
                    .details(&format!(
                        "{} - {}",
                        curr_playing.title,
                        curr_playing.aux_title.clone().unwrap()
                    ))
                    // .assets(Assets::new().large_image(&curr_playing.metadata_media.clone().unwrap()))
                    .state(
                        format!(
                            "{} played",
                            pretty_time(
                                curr_playing
                                    .progress
                                    .unwrap_or(Duration::from_secs(time_elapsed))
                            )
                        )
                        .as_str(),
                    ),
            )
            .unwrap();
        //}
        time_elapsed += 3;
        thread::sleep(Duration::from_secs(3))
    }
}
