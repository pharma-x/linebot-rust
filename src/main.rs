use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    //logging
    let log_level = env::var("Rust_LOG").unwrap_or("info".to_string());
    env::set_var("Rust_LOG", log_level);
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/linebot-webhook", post(linebot_handler));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello World!"
}

async fn linebot_handler(Json(payload): Json<LineBotRequest>) -> impl IntoResponse {
    let events = payload.events;

    (StatusCode::CREATED, Json(events));
}

#[derive(Deserialize)]
struct LineBotRequest {
    destination: String,
    events: Vec<LineBotEvent>,
}

#[derive(Deserialize)]
struct LineBotEvent {
    r#type: String,
    mode: String,
    timestamp: u64,
    source: LineBotSource,
    reply_token: String,
    message: LineBotMessage,
}

#[derive(Deserialize)]
struct LineBotSource {
    r#type: String,
    user_id: String,
}

#[derive(Deserialize)]
struct LineBotMessage {
    id: String,
    r#type: String,
    text: String,
}
