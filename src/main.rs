mod metadata_providers;
mod settings;
mod util;
use anyhow::anyhow;
use discord_rich_presence::{
    activity::{Activity, Assets},
    DiscordIpc, DiscordIpcClient,
};
use dotenvy::dotenv;
use fltk::{
    app::{self},
    button::{Button, CheckButton},
    input::Input,
    prelude::*,
    text::TextDisplay,
    window::Window,
};
use settings::AppSettings;
use std::{
    error::Error,
    process::Command,
    sync::{mpsc, Arc, RwLock},
    thread,
    time::{Duration, Instant},
};
use tokio::task;
use tray_icon::{
    menu::{Menu, MenuEvent, MenuItemKind::MenuItem},
    Icon, TrayIconBuilder, TrayIconEvent,
};
use util::pretty_time;

use metadata_providers::{mpris_parser::MprisParser, windows::WindowsParser, MetadataProvider};

use crate::metadata_providers::extension::run_server;

const DELAY_TO_RECHECK: u64 = 3;
const UI_DEFAULT_WIDTH: i32 = 300;
const UI_DEFAULT_HEIGHT: i32 = 30;
const IPC_WAITING_TIMEOUT: Duration = Duration::from_secs(60);

const DISCORD_ID: &str = env!("DISCORD_ID");

fn restart_service() -> Result<(), anyhow::Error> {
    let path = std::env::current_dir()?;
    Command::new(path);
    std::process::exit(0);
}

fn get_os_specific_provider() -> Option<Box<dyn MetadataProvider>> {
    // in the meantime, use only the first metadata provider.
    if cfg!(target_os = "linux") {
        Some(Box::new(MprisParser::new()))
    } else if cfg!(target_os = "windows") {
        Some(Box::new(WindowsParser::default()))
    } else {
        None
    }
}

fn ui(settings: Arc<RwLock<AppSettings>>) -> anyhow::Result<()> {
    let _settings_reader = settings
        .read()
        .map_err(|_err| anyhow!("Failed to acquire a lock"));
    let app = app::App::default().with_scheme(fltk::app::AppScheme::Gtk);
    let mut wind = Window::default().with_size(500, 200).center_screen();

    let save_label = TextDisplay::default()
        .with_pos(150, 25)
        .with_label("App needs to quit after making changes.");

    let mut extension_enabled_textbox = CheckButton::default()
        .with_label("Enable Extension Host")
        .with_size(UI_DEFAULT_WIDTH, UI_DEFAULT_HEIGHT)
        .below_of(&save_label, 2);
    extension_enabled_textbox.set_callback(|_| {
        // handle this later
    });
    let mut jellyfin_token_textbox = Input::default()
        .with_label("Jellyfin Cookie")
        .with_size(UI_DEFAULT_WIDTH, UI_DEFAULT_HEIGHT)
        .below_of(&extension_enabled_textbox, 2);
    jellyfin_token_textbox.set_callback(move |_| {
        // handle this later
    });
    let mut save_button = Button::default()
        .with_label("Save Changes")
        .with_size(UI_DEFAULT_WIDTH, UI_DEFAULT_HEIGHT)
        .below_of(&jellyfin_token_textbox, 10);
    let service_status_label = TextDisplay::default()
        .with_label("Service Status")
        .with_size(UI_DEFAULT_WIDTH, UI_DEFAULT_HEIGHT)
        .below_of(&save_button, 3);

    save_button.set_callback(|_| {
        let _ = restart_service();
    });
    wind.add(&save_button);
    wind.add(&save_label);
    wind.add(&extension_enabled_textbox);
    wind.add(&jellyfin_token_textbox);
    wind.add(&service_status_label);
    wind.end();
    wind.show();

    let _ = app.run();

    Ok(())
}

fn service(settings: Arc<RwLock<AppSettings>>) -> anyhow::Result<()> {
    let settings: std::sync::RwLockReadGuard<'_, AppSettings> = settings
        .read()
        .map_err(|_error| anyhow!("Failed to acquire a lock on app settings"))?;
    let mut provider: Option<Box<dyn MetadataProvider>> = None;
    if settings
        .metadata_sources
        .contains(&"native_now_playing".to_string())
    {
        provider = get_os_specific_provider();
    }
    let timer = Instant::now();
    let mut ipc_client: DiscordIpcClient = DiscordIpcClient::new(DISCORD_ID);
    println!("Looking for Discord IPC Client");
    while !ipc_client.connect().is_ok() {
        if timer.elapsed() > IPC_WAITING_TIMEOUT {
            return Err(anyhow!("Failed to find Discord IPC"));
        }
    }
    println!("IPC Client Found!");
    let mut time_elapsed: u64 = 0;

    loop {
        #[allow(clippy::expect_used)]
        let playing_metadata = provider
            .as_ref()
            .expect("No provider was found for playing metadata")
            .get_playing_metadata(&settings);
        let Some(playing_metadata) = playing_metadata else {
            continue;
        };
        println!("{:?}", playing_metadata);

        let _activity = ipc_client.set_activity(
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
                .name(
                    playing_metadata
                        .subprovider_name
                        .unwrap_or(String::from("Media")),
                )
                .assets(
                    Assets::new()
                        .large_image(playing_metadata.metadata_media.clone().unwrap_or(
                            String::from("https://cdn.frankerfacez.com/emoticon/660211/4"),
                        ))
                        .small_image(playing_metadata.metadata_media.clone().unwrap_or(
                            String::from("https://cdn.frankerfacez.com/emoticon/660211/4"),
                        )),
                )
                .details(format!(
                    "{} - {}",
                    playing_metadata.title,
                    playing_metadata.aux_title.unwrap_or_default()
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
        time_elapsed += DELAY_TO_RECHECK;
        thread::sleep(Duration::from_secs(DELAY_TO_RECHECK));
    }
}

#[tokio::main]
async fn main() {
    let app = app().await;

    match app {
        Ok(()) => {},
        Err(err) => println!("Failed to run drp.rs. Error: {:?}", err)
    };
}
async fn app() -> anyhow::Result<()>{
    let _ = dotenvy::dotenv()?;
    let data = Arc::new(RwLock::new(AppSettings::new()?));
    let ui_lock = Arc::clone(&data);
    let extension_lock = Arc::clone(&data);
    let service_lock = Arc::clone(&data);

    let icon_image = image::open("assets/play.png")?;
    let menu = Menu::new();
    let _ = menu.append_items(&[&tray_icon::menu::MenuItem::new("Menu item #3", true, None)]);
    let icon = Icon::from_rgba(
        icon_image.as_bytes().to_vec(),
        icon_image.width(),
        icon_image.height(),
    )?;
    let tray_icon = TrayIconBuilder::new()
        .with_tooltip("system-tray - tray icon library!")
        .with_icon(icon)
        .with_menu(Box::new(menu))
        .build()?;
    task::spawn_blocking(|| ui(ui_lock));
    task::spawn_blocking(|| service(service_lock));

    if extension_lock
        .read()
        .map_err(|_| anyhow!("Could not read settings"))?
        .extension_host_enabled
    {
        tokio::spawn(async { run_server().await });
    }

    // handle tray events
    loop {
        if let Ok(event) = TrayIconEvent::receiver().try_recv() {
            println!("tray event: {:?}", event);
        }

        if let Ok(event) = MenuEvent::receiver().try_recv() {
            println!("menu event: {:?}", event);
        }
    }
}
