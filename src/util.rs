use std::time::Duration;

use crate::metadata_providers::MediaType;
pub fn get_action_string(activity_type: &MediaType, subtitle: &str) -> String {
    match activity_type {
        MediaType::Audio => format!("Listening to {}", subtitle),
        MediaType::Video => todo!("Watching {}", subtitle),
        _ => subtitle.to_string(),
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
