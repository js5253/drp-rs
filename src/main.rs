#![deny(clippy::unwrap_used)]
mod metadata_providers;
mod settings;
mod util;
use anyhow::anyhow;
use discord_rich_presence::{
    activity::{Activity, ActivityType, Assets},
    DiscordIpc, DiscordIpcClient,
};
use dotenvy::dotenv;
use fltk::{
    app::{self},
    button::{Button, CheckButton},
    prelude::*,
    text::TextDisplay,
    window::Window,
};
use futures::executor::block_on;
use image::ImageReader;
use settings::AppSettings;
use std::{
    process::Command,
    time::{Duration, Instant},
};
use std::{sync::Arc, thread};
use tokio::{
    signal,
    sync::{mpsc, RwLock},
    task::spawn_blocking,
};
use tokio_util::{sync::CancellationToken, task::TaskTracker};
use tracing_subscriber::EnvFilter;
use tray_icon::{menu::MenuEvent, Icon, TrayIconBuilder, TrayIconEvent};

use util::pretty_time;

use metadata_providers::{mpris_parser::MprisParser, windows::WindowsParser, MetadataProvider};

use crate::{
    metadata_providers::{extension::ExtensionProvider, MediaType, Metadata},
    util::get_action_string,
};
const DELAY_TO_RECHECK: u64 = 3;
const UI_DEFAULT_WIDTH: i32 = 300;
const UI_DEFAULT_HEIGHT: i32 = 30;
const IPC_WAITING_TIMEOUT: Duration = Duration::from_secs(60);

const LISTENING_ICON_URL: &str = "https://raw.githubusercontent.com/js5253/drp-rs/refs/heads/main/assets/CD.svg?token=GHSAT0AAAAAAEE36YXNCUT4XT5CS7BPZVE62UB6J5Q";
const WATCHING_ICON_URL: &str = "https://raw.githubusercontent.com/js5253/drp-rs/refs/heads/main/assets/TV.svg?token=GHSAT0AAAAAAEE36YXNTSBLSWHMRP2GYJC42UB6KSQ";
const PLAYING_ICON_URL: &str = "https://raw.githubusercontent.com/js5253/drp-rs/refs/heads/main/assets/Media.svg?token=GHSAT0AAAAAAEE36YXNRCMTOTAZ2P3V46TQ2UB6MGQ";

const DISCORD_ID: &str = env!("DISCORD_ID");

fn restart_service() -> Result<(), anyhow::Error> {
    let path = std::env::current_dir()?;
    Command::new(path);
    std::process::exit(0);
}

fn get_os_specific_provider() -> Option<Box<dyn MetadataProvider + Send>> {
    // in the meantime, use only the first metadata provider.
    if cfg!(target_os = "linux") {
        Some(Box::new(MprisParser::new()))
    } else if cfg!(target_os = "windows") {
        Some(Box::new(WindowsParser::default()))
    } else {
        None
    }
}
async fn init_providers(
    tx: mpsc::Sender<Metadata>,
    settings: Arc<RwLock<AppSettings>>,
    token: CancellationToken,
) {
    let settings = settings.read().await;
    if settings
        .metadata_sources
        .contains(&"native_now_playing".to_string())
    {
        match get_os_specific_provider() {
            Some(provider) => {
                provider.register_thread(tx.clone(), token.clone());
            }
            None => {}
        }
    }
    if settings.metadata_sources.contains(&"extension".to_string()) {
        ExtensionProvider::default().register_thread(tx.clone(), token);
    }
}

async fn service(
    settings: Arc<RwLock<AppSettings>>,
    token: CancellationToken,
    mut rx: mpsc::Receiver<Metadata>,
) -> anyhow::Result<()> {
    let timer = Instant::now();
    let mut ipc_client: DiscordIpcClient = DiscordIpcClient::new(DISCORD_ID);
    while !ipc_client.connect().is_ok() {
        if timer.elapsed() > IPC_WAITING_TIMEOUT {
            return Err(anyhow!("Failed to find Discord IPC"));
        }
    }
    let mut time_elapsed: u64 = 0;
    let mut prev_playing: Option<Metadata> = None;
    while let Some(playing_metadata) = rx.recv().await {
        if token.is_cancelled() {
            return ipc_client
                .close()
                .map_err(|_| anyhow!("could not close discord ipc?"));
        }

        if prev_playing
            .as_ref()
            .is_some_and(|metadata| *metadata == playing_metadata)
        {
            continue;
        }

        let (activity_type, default_image) = match playing_metadata.media_type {
            MediaType::AUDIO => (ActivityType::Listening, String::from(LISTENING_ICON_URL)), // replace these with GitHub hosted images maybe?
            MediaType::VIDEO => (ActivityType::Watching, String::from(WATCHING_ICON_URL)),
            MediaType::MIXED => (ActivityType::Playing, String::from(PLAYING_ICON_URL)),
        };

        let _activity = ipc_client.set_activity(
            Activity::new()
                .activity_type(activity_type)
                .name(get_action_string(
                    &playing_metadata.media_type,
                    playing_metadata.title.clone(),
                ))
                .assets(
                    Assets::new()
                        .large_image(
                            playing_metadata
                                .metadata_media
                                .clone()
                                .unwrap_or(default_image.clone()),
                        )
                        .large_image(
                            playing_metadata
                                .metadata_media
                                .clone()
                                .unwrap_or(default_image),
                        ),
                )
                .details(format!(
                    "{} - {}",
                    playing_metadata.title,
                    playing_metadata.aux_title.clone().unwrap_or_default()
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
        prev_playing = Some(playing_metadata.clone());
        time_elapsed += DELAY_TO_RECHECK;
        tokio::time::sleep(Duration::from_secs(DELAY_TO_RECHECK)).await;
    }
    Ok(())
}

fn main() {
    let _unused = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .name("my-runtime")
        .build()
        .expect("Could not launch tokio runtime")
        .block_on(async { app().await });
}
async fn app() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv()?;
    tracing_subscriber::fmt()
        // This allows you to use, e.g., `RUST_LOG=info` or `RUST_LOG=debug`
        // when running the app to set log levels.
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("drp-rs=error,tower_http=warn"))
                .expect("Failed to set up logging"),
        )
        .init();
    let tracker = TaskTracker::new();
    let token = CancellationToken::new();
    let data = Arc::new(RwLock::new(AppSettings::load_or_default()));

    let ui_lock = Arc::clone(&data);
    let extension_lock = Arc::clone(&data);
    let service_lock = Arc::clone(&data);

    let service_token = token.clone();
    let extension_token = token.clone();
    let ui_token = token.clone();

    let (mut tx, rx) = mpsc::channel(100);
    init_providers(tx, service_lock, service_token).await;

    // ui can't be moved outside the main thread; there's probably a way to solve this.
    let _settings_reader = block_on(async { ui_lock.read().await });
    let app = app::App::default().with_scheme(fltk::app::AppScheme::Gtk);
    let mut wind = Window::default().with_size(500, 200).center_screen();

    let mut extension_enabled_textbox = CheckButton::default()
        .with_label("Enable Extension Host")
        .with_size(UI_DEFAULT_WIDTH, UI_DEFAULT_HEIGHT)
        .center_of_parent();
    extension_enabled_textbox.set_callback(|_| {
        // handle this later
    });

    let mut save_button = Button::default()
        .with_label("Save Changes")
        .with_size(UI_DEFAULT_WIDTH, UI_DEFAULT_HEIGHT)
        .below_of(&extension_enabled_textbox, 10);
    let service_status_label = TextDisplay::default()
        .with_label("Service Status")
        .with_size(UI_DEFAULT_WIDTH, UI_DEFAULT_HEIGHT)
        .below_of(&save_button, 3);

    save_button.set_callback(|_| {
        let _ = restart_service();
    });
    wind.add(&save_button);
    wind.add(&extension_enabled_textbox);
    wind.add(&service_status_label);
    wind.end();
    wind.show();
    let _ = app.run();
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("path")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let tray_icon = TrayIconBuilder::new()
        .with_tooltip("system-tray - tray icon library!")
        .with_icon(Icon::from_rgba(icon_rgba, icon_width, icon_height)?)
        .build()?;
    if let Ok(event) = TrayIconEvent::receiver().try_recv() {
        println!("tray event: {:?}", event);
    }

    if let Ok(event) = MenuEvent::receiver().try_recv() {
        println!("menu event: {:?}", event);
    } // end ui section here
      // tracker.spawn(async { ui(ui_lock, ui_token).await });
    tracker.spawn(async { service(extension_lock, extension_token, rx).await });

    tracker.close();
    tracker.wait().await;
    match signal::ctrl_c().await {
        Ok(()) => {
            token.cancel();
        }
        Err(err) => {}
    }
    Ok(())
}
