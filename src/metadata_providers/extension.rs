// use axum::{
//     routing::{post},
//     extract::Json,
//      Router,
// };
// use serde::{Deserialize, Serialize};


// pub async fn main() {
//     let app = Router::new().route("/register_change", post(register_change));
//     axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
//     }
// #[derive(Serialize, Deserialize, Debug)]
// struct AppRequest {
//     provider_name: String,
//     title: String,
//     artist: String,
// }

// async fn register_change(Json(payload): Json<AppRequest>) -> &'static str {
//     println!("{:?}", payload);
//     "OK"
// }
