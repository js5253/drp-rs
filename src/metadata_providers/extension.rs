use std::env;

use anyhow::Error;
use axum::{extract::Json, routing::post, Extension, Router};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{self, Sender};
use tokio_util::sync::CancellationToken;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

use crate::metadata_providers::{Metadata, MetadataProvider};

#[derive(Default)]
pub struct ExtensionProvider;

#[derive(Deserialize, Debug)]
pub struct RegisterActivityBody {
    subprovider_name: String,
    title: String,
    artist: String,
}
#[derive(Clone)]
struct AppState(Sender<Metadata>);
pub async fn register_activity(Json(body): Json<RegisterActivityBody>) {
    // figure things out later here;


}

async fn run_server(tx: mpsc::Sender<Metadata>, token: CancellationToken) -> Result<(), Error> {
    println!("Running drp.rs server...");
    let addr_to_bind = env::var("ADDR_TO_BIND").unwrap_or("0.0.0.0:3000".to_string());

    let listener = tokio::net::TcpListener::bind(addr_to_bind.clone())
        .await
        .unwrap();
    println!("listening on {}", addr_to_bind);

    let app = Router::new()
        .route("/register_activity", post(register_activity))
        .layer(TraceLayer::new_for_http())
        .with_state(AppState(tx));
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

impl MetadataProvider for ExtensionProvider {
    fn register_thread(&self, tx: mpsc::Sender<Metadata>, token: CancellationToken) {
        println!("Got to MetadataProvider");
        tokio::spawn(async {
            run_server(tx, token).await
        });
    }
}