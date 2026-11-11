use std::env;

use anyhow::Error;
use axum::{Extension, Router, extract::{Json, State}, routing::post};
use dotenvy::dotenv;
use mpris::PlaybackStatus;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{self, Sender};
use tokio_util::sync::CancellationToken;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

use crate::metadata_providers::{Metadata, MetadataProvider};

pub const WEB_EXTENSION_STRING: &str = "extension";

#[derive(Default)]
pub struct ExtensionProvider;

#[derive(Deserialize, Debug)]
pub struct RegisterActivityBody {
    subprovider_name: String,
    title: String,
    artist: String,
    media_type: String
}
#[derive(Clone)]
pub struct AppState(Sender<Metadata>);
pub async fn register_activity(State(state): State<AppState>, Json(body): Json<RegisterActivityBody>) {
    // figure things out later here;
    let _ = state.0.send(Metadata { title: body.title
        , aux_title: None, progress: None, state: Some(super::PlaybackState::PAUSED), provider_name: "Web Extension".to_string(), subprovider_name: Some(body.subprovider_name), media_type: super::MediaType::MIXED, metadata_media: None }).await;
}

async fn run_server(tx: mpsc::Sender<Metadata>, token: CancellationToken) -> Result<(), Error> {
    println!("Running drp.rs server...");
    let addr_to_bind = env::var("ADDR_TO_BIND").unwrap_or("0.0.0.0:3000".to_string());

    let listener = tokio::net::TcpListener::bind(addr_to_bind.clone())
        .await?;
    println!("listening on {}", addr_to_bind);

    let app = Router::new()
        .route("/register_activity", post(register_activity))
        .layer(TraceLayer::new_for_http())
        .with_state(AppState(tx));
    axum::serve(listener, app).await?;
    Ok(())
}

impl MetadataProvider for ExtensionProvider {
    fn register_thread(&self, tx: mpsc::Sender<Metadata>, token: CancellationToken) {
        tokio::spawn(async {
            run_server(tx, token).await
        });
    }
}