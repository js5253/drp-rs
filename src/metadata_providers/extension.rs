use std::env;

use anyhow::Error;
use axum::{extract::Json, routing::post, Extension, Router};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

use crate::metadata_providers::Metadata;

#[derive(Deserialize, Debug)]
pub struct RegisterActivityBody {
    subprovider_name: String,
    title: String,
    artist: String,
}
pub async fn register_activity(Json(body): Json<RegisterActivityBody>) {
    // figure things out later here;
}

pub async fn run_server(mut tx: mpsc::Sender<Metadata>, token: CancellationToken) -> Result<(), Error> {
    println!("Running drp.rs server...");
    let addr_to_bind = env::var("ADDR_TO_BIND").unwrap_or("0.0.0.0:3000".to_string());

    let listener = tokio::net::TcpListener::bind(addr_to_bind.clone())
        .await
        .unwrap();
    println!("listening on {}", addr_to_bind);

    let app = Router::new()
        .route("/register_activity", post(register_activity))
        .layer(TraceLayer::new_for_http());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
