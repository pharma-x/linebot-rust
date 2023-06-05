pub mod adapter;
pub mod application;
pub mod domain;
pub mod presentation;
use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use presentation::module::Modules;
use presentation::routes::line_webhook::line_webhook_handler;
use std::env;
use std::{net::SocketAddr, sync::Arc};

#[tokio::main]
async fn main() {
    //logging
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);

    init_app();

    // DI
    let modules = Modules::new().await;

    let root = Router::new().route("/", get(root));
    let line_webhook_router = Router::new().route("/", post(line_webhook_handler));

    let app = Router::new()
        .nest("/", root)
        .nest("/linebot-webhook", line_webhook_router)
        .layer(Extension(Arc::new(modules)));

    // localhost:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::debug!("Server listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|_| panic!("Server cannot launch!"))
}

async fn root() -> &'static str {
    "Hello World!"
}

pub fn init_app() {
    dotenv().ok();
    tracing_subscriber::fmt::init();
}
