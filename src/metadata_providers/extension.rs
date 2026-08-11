use std::env;

use axum::{extract::Json, routing::post, Extension, Router};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

#[derive(Deserialize)]
struct RegisterActivityBody {
    title: String,
    aux_title: String

}
pub async fn register_activity(Json(body): Json<RegisterActivityBody>) {
    // figure things out later here;

}

pub async fn main() {
    dotenv().expect("Missing .env - copy .env.example and fill out.");
    let addr_to_bind = env::var("ADDR_TO_BIND").unwrap_or("0.0.0.0:3000".to_string());

    tracing_subscriber::fmt()
        // This allows you to use, e.g., `RUST_LOG=info` or `RUST_LOG=debug`
        // when running the app to set log levels.
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("spotilocker_2=error,tower_http=warn"))
                .unwrap(),
        )
        .init();

    let listener = tokio::net::TcpListener::bind(addr_to_bind.clone())
        .await
        .unwrap();
    println!("listening on {}", addr_to_bind);

    let app = Router::new()
        .route("/register_activity", post(register_activity))
        .layer(TraceLayer::new_for_http());
    axum::serve(listener, app).await.unwrap();
}
#[derive(Serialize, Deserialize, Debug)]
struct AppRequest {
    provider_name: String,
    title: String,
    artist: String,
}
