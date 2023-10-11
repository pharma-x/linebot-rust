use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use presentation::{module::Modules, routes::line_webhook::line_webhook_handler};
use std::env;
use std::{net::SocketAddr, sync::Arc};

#[tokio::main]
async fn main() {
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
    //logging
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();
}

#[cfg(test)]
mod test {
    use super::*;
    use axum_test::{
        http::header::{HeaderName, HeaderValue},
        TestServer,
    };
    use presentation::model::line_webhook::LineWebhookRequests;

    #[tokio::test]
    async fn test_linebot_webhook() {
        init_app();
        // DI
        let modules = Modules::new().await;
        /*
         * テスト用のサーバーを作成する
         */
        let line_webhook_router = Router::new().route("/", post(line_webhook_handler));
        let test_app = Router::new()
            .nest("/linebot-webhook", line_webhook_router)
            .layer(Extension(Arc::new(modules)));
        let test_server = TestServer::new(test_app.into_make_service()).unwrap();

        let request =
            LineWebhookRequests::new("U00000000000000000000000000000000".to_string(), vec![]);

        let response = test_server
            .post("/linebot-webhook")
            .add_header(
                HeaderName::from_lowercase(b"x_line_signature").unwrap(),
                HeaderValue::from_str("test_signature").unwrap(),
            )
            .json(&request)
            .await;

        response.assert_status_ok();
    }
}
