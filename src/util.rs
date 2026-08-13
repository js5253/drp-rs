use std::time::Duration;

use discord_rich_presence::activity::ActivityType;

use crate::metadata_providers::MediaType;
pub fn get_action_string(activity_type: &MediaType, subtitle: String) -> String {
    match activity_type {
        MediaType::AUDIO => format!("Listening to {}", subtitle),
        MediaType::VIDEO => todo!("Watching {}", subtitle),
        _ => subtitle.clone(),
    }
}

pub fn pretty_time(dur: Duration) -> String {
    let seconds = dur.as_secs();
    let minutes = seconds / 60;
    let remaining_seconds = seconds % 60;
    format!("{:02}:{:02}", minutes, remaining_seconds)
}

pub fn get_playback_sign(status: &str) -> &str {
    match status {
        "paused" => "⏸︎",
        "playing" => "⏵︎",
        _ => "",
    }
}
