use std::env;

use anyhow::Error;
use axum::{
    extract::{Json, State},
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use tokio::sync::mpsc::{self, Sender};
use tokio_util::sync::CancellationToken;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;

use crate::metadata_providers::{MediaType, Metadata, MetadataProvider};

#[derive(Default)]
pub struct ExtensionProvider;

#[derive(Deserialize, Debug)]
pub struct RegisterActivityBody {
    subprovider_name: String,
    title: String,
    artist: Option<String>,
    media_type: MediaType,
}
#[derive(Clone)]
pub struct AppState(Sender<Metadata>);
pub async fn register_activity(
    State(state): State<AppState>,
    Json(body): Json<RegisterActivityBody>,
) {
    // figure things out later here;
    let _ = state
        .0
        .send(Metadata {
            title: body.title,
            aux_title: body.artist,
            progress: None,
            state: Some(super::PlaybackState::Playing),
            provider_name: "Web Extension".to_string(),
            subprovider_name: Some(body.subprovider_name),
            media_type: body.media_type,
            metadata_media: None,
        })
        .await;
}
pub async fn heartbeat() -> &'static str {
    "drp.rs"
}

async fn run_server(tx: mpsc::Sender<Metadata>, token: CancellationToken) -> Result<(), Error> {
    info!("Running drp.rs server...");
    let addr_to_bind = env::var("ADDR_TO_BIND").unwrap_or("0.0.0.0:3000".to_string());

    let listener = tokio::net::TcpListener::bind(addr_to_bind.clone()).await?;
    println!("listening on {}", addr_to_bind);

    let app = Router::new()
        .route("/register_activity", post(register_activity))
        .route("/", get(heartbeat))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(AppState(tx));
    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            token.cancelled().await;
        })
        .await?;
    Ok(())
}

impl MetadataProvider for ExtensionProvider {
    fn register_thread(&self, tx: mpsc::Sender<Metadata>, token: CancellationToken) {
        tokio::spawn(async { run_server(tx, token).await });
    }
}
