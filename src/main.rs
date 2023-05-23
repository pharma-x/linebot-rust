pub mod adapter;
pub mod application;
pub mod domain;
pub mod presentation;
use axum::{
    routing::{get, post},
    Router,
};
use presentation::routes::line_webhook;
use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    //logging
    let log_level = env::var("Rust_LOG").unwrap_or("info".to_string());
    env::set_var("Rust_LOG", log_level);
    tracing_subscriber::fmt::init();

    let line_webhook_router = Router::new().route("/", post(line_webhook::handler));

    let app = Router::new()
        .route("/", get(root))
        .nest("/linebot-webhook", line_webhook_router);

    // localhost:3000
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
