mod metadata_providers;
mod settings;
mod util;
use discord_rich_presence::{
    activity::{Activity, Assets},
    DiscordIpc, DiscordIpcClient,
};
use fltk::{
    app::{self},
    button::Button,
    input::Input,
    prelude::*,
    text::TextDisplay,
    window::Window,
};
use settings::AppSettings;
use std::{
    error::Error,
    process::Command,
    sync::{Arc, Mutex, RwLock},
    thread,
    time::Duration,
};
use util::pretty_time;

use metadata_providers::{mpris_parser::MprisParser, windows::WindowsParser, MetadataProvider};

use lazy_static::lazy_static;

const DELAY_TO_RECHECK: u64 = 3;
const DISCORD_ID: &str = "1162169068418248764";

const UI_DEFAULT_WIDTH: i32 = 300;
const UI_DEFAULT_HEIGHT: i32 = 30;

fn restart_service() -> Result<(), anyhow::Error> {
    let path = std::env::current_dir()?;
    Command::new(path);
    std::process::exit(0);
    Ok(())
}

fn get_os_specific_provider() -> Option<Box<dyn MetadataProvider>> {
    // in the meantime, use only the first metadata provider.
    if cfg!(linux) {
        Some(Box::new(MprisParser::new()))
    } else if cfg!(windows) {
        Some(Box::new(WindowsParser::default()))
    } else {
        return None;
    }
}

fn ui(settings: Arc<RwLock<AppSettings>>) {
    println!("{:?}", settings);
    let settings_reader = settings.read().unwrap();
    let app = app::App::default().with_scheme(fltk::app::AppScheme::Gtk);
    let mut wind = Window::default().with_size(500, 200).center_screen();

    let save_label = TextDisplay::default()
        .with_pos(150, 25)
        .with_label("App needs to quit after making changes.");
    let mut tautulli_token_textbox = Input::default()
        .with_label("Tautulli Cookie")
        .with_size(UI_DEFAULT_WIDTH, UI_DEFAULT_HEIGHT)
        .below_of(&save_label, 2);
    let _ = &tautulli_token_textbox.set_value(&settings_reader.tautulli_server_cookie);
    let _ = &tautulli_token_textbox.set_callback(move |data| {
        // handle this later
    });

    let mut jellyfin_token_textbox = Input::default()
        .with_label("Jellyfin Cookie")
        .with_size(UI_DEFAULT_WIDTH, UI_DEFAULT_HEIGHT)
        .below_of(&tautulli_token_textbox, 2);
    &jellyfin_token_textbox.set_callback(move |data| {
        // handle this later
    });
    let mut save_button = Button::default()
        .with_label("Save Changes")
        .with_size(UI_DEFAULT_WIDTH, UI_DEFAULT_HEIGHT)
        .below_of(&jellyfin_token_textbox, 10);
    let mut service_status_label = TextDisplay::default()
        .with_label("Service Status")
        .with_size(UI_DEFAULT_WIDTH, UI_DEFAULT_HEIGHT)
        .below_of(&save_button, 3);

    save_button.set_callback(|_| {
        restart_service().unwrap();
    });
    wind.add(&tautulli_token_textbox);
    wind.add(&save_button);
    wind.add(&service_status_label);
    wind.end();
    wind.show();
    app.run().unwrap();
}

fn service(settings: Arc<RwLock<AppSettings>>) -> Result<(), Box<dyn Error + Send>> {
    let settings = settings.read().unwrap();
    println!("{:?}", *settings);
    let mut provider: Option<Box<dyn MetadataProvider>> = None;
    if settings
        .metadata_sources
        .contains(&"native_now_playing".to_string())
    {
        provider = Some(
            get_os_specific_provider()
                .expect("Could not find a default provider for your platform"),
        );
    }
    let mut ipc_client: DiscordIpcClient = DiscordIpcClient::new(DISCORD_ID);
    ipc_client.connect().unwrap();
    loop {
        let mut time_elapsed: u64 = 0;

        println!("Something here...");
        let prev_playing = provider.expect("Could not find a provider...");
        println!("Discord Playing Thing");
        loop {
            let playing_metadata = prev_playing
                .get_playing_metadata(&settings)
                .expect("Could not find metadata");
            let activity = ipc_client.set_activity(
                Activity::new()
                    .activity_type(match playing_metadata.media_type {
                        metadata_providers::MediaType::AUDIO => {
                            discord_rich_presence::activity::ActivityType::Listening
                        }
                        metadata_providers::MediaType::VIDEO => {
                            discord_rich_presence::activity::ActivityType::Watching
                        }
                        metadata_providers::MediaType::MIXED => {
                            discord_rich_presence::activity::ActivityType::Playing
                        }
                    })
                    .assets(
                        Assets::new()
                            .large_image("https://cdn.frankerfacez.com/emoticon/660211/4")
                            .small_image("https://cdn.frankerfacez.com/emoticon/660211/4"),
                    )
                    .details(&format!(
                        "{} - {}",
                        &playing_metadata.title,
                        &playing_metadata.aux_title.unwrap_or_default()
                    ))
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
            );
            activity.unwrap();
            time_elapsed += DELAY_TO_RECHECK;
            thread::sleep(Duration::from_secs(DELAY_TO_RECHECK));
        }
    }
}
#[derive(Debug)]
enum AppStatus {
    STOPPED,
    RUNNING,
    ERROR,
}

fn main() -> Result<(), Box<dyn Error + Send>> {
    let data = Arc::new(RwLock::new(
        AppSettings::new().expect("Could not read settings file."),
    ));
    let a1 = Arc::clone(&data);
    let a2 = Arc::clone(&data);
    let _ = thread::spawn(move || service(a2).unwrap()).join();
    ui(a1);
    Ok(())
}
