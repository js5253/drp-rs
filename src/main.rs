mod util;
mod metadata_providers;
mod settings;
use util::pretty_time;
use discord_rich_presence::{
    activity::{Activity, Assets},
    DiscordIpc, DiscordIpcClient,
};
use settings::AppSettings;
use std::{error::Error, sync::{Arc, Mutex}, thread, time::Duration};
use fltk::{
    app::{self},
    button::Button,
    input::Input,
    prelude::*,
    text::TextDisplay,
    window::Window,
};
use metadata_providers::{mpris::MprisParser, windows::WindowsParser, MetadataProvider};

use lazy_static::lazy_static;

const DELAY_TO_RECHECK: u64 = 3;
const DISCORD_ID: &str = "1162169068418248764";

const UI_DEFAULT_WIDTH: i32 = 300;
const UI_DEFAULT_HEIGHT: i32 = 30;
fn get_os_specific_provider() -> Option<Box<dyn MetadataProvider>> {
    // in the meantime, use only the first metadata provider.
    if cfg!(linux) {
        Some(Box::new(MprisParser::default()))
    } else if cfg!(windows) {
        Some(Box::new(WindowsParser::default()))
    } else {
        return None;
    }
}

lazy_static! {
    pub static ref SETTINGS: AppSettings = AppSettings::new().expect("Config file is incorrect.");
}


fn ui(thread_arc: Arc<Mutex<AppStatus>>) {
    println!("{:?}", thread_arc);
    let app = app::App::default().with_scheme(fltk::app::AppScheme::Gtk);
    let mut wind = Window::default().with_size(500, 200).center_screen();

    let mut save_label = TextDisplay::default()
        .with_pos(150, 25)
        .with_label("App needs to quit after making changes.");
    let mut tautulli_token_textbox = Input::default()
        .with_label("Tautulli Cookie")
        .with_size(UI_DEFAULT_WIDTH, UI_DEFAULT_HEIGHT)
        .below_of(&save_label, 2);
    let mut tautulli_token_state = String::from(&SETTINGS.tautulli_server_cookie);
    let _ = &tautulli_token_textbox.set_value(&tautulli_token_state);
    let _ = &tautulli_token_textbox.set_callback(move |data| {
        tautulli_token_state = data.value();
    });

    let mut jellyfin_token_textbox = Input::default()
        .with_label("Jellyfin Cookie")
        .with_size(UI_DEFAULT_WIDTH, UI_DEFAULT_HEIGHT)
        .below_of(&tautulli_token_textbox, 2);
    let mut jellyfin_token_state = String::new();
    &jellyfin_token_textbox.set_callback(move |data| {
        jellyfin_token_state = data.value();
    });
    let mut save_button = Button::default()
        .with_label("Save Changes")
        .with_size(UI_DEFAULT_WIDTH, UI_DEFAULT_HEIGHT)
        .below_of(&jellyfin_token_textbox, 10);
    let mut service_status_label = TextDisplay::default()
        .with_label("Service Status")
        .with_size(UI_DEFAULT_WIDTH, UI_DEFAULT_HEIGHT)
        .below_of(&save_button, 3);
    wind.add(&tautulli_token_textbox);
    wind.add(&save_button);
    wind.add(&service_status_label);
    wind.end();
    wind.show();
    app.run().unwrap();
}

fn service(app_status: Arc<Mutex<AppStatus>>) -> Result<(), Box<dyn Error + Send>> {
    {
        let mut data = app_status.lock().unwrap();
        *data = AppStatus::RUNNING;

        println!("{:?}", &data);
    }
    println!("{:?}", SETTINGS.metadata_sources);
    let mut provider: Option<Box<dyn MetadataProvider>> = None;
    if SETTINGS
        .metadata_sources
        .contains(&"native_now_playing".to_string())
    {
        provider = Some(get_os_specific_provider().expect("Could not find a default provider"));
    }

    loop {
        let mut ipc_client: DiscordIpcClient =
            DiscordIpcClient::new(DISCORD_ID).expect("Could not connect to Discord");
        ipc_client.connect().unwrap();
        let mut time_elapsed: u64 = 0;

        println!("Something here...");
        let prev_playing = provider.expect("Could not find a provider...");
            println!("Discord Playing Thing");
            loop {
                let playing_metadata = prev_playing.get_playing_metadata().expect("Could not find metadata");
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
    let data = Arc::new(Mutex::new(AppStatus::STOPPED));
    let a1 = Arc::clone(&data);
    let a2 = Arc::clone(&data);
    thread::spawn(move || service(a2).unwrap()).join();
    ui(a1);
    Ok(())
}
