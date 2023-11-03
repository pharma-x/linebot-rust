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
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

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
    use base64::{engine::general_purpose, Engine as _};
    use hmac::{Hmac, Mac};
    use presentation::model::line_webhook::LineWebhookRequests;
    use sha2::Sha256;

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
        /*
         * signatureを作成する
         */
        let request =
            LineWebhookRequests::new("U00000000000000000000000000000000".to_string(), vec![]);
        let channel_secret = env::var("LINE_CHANNEL_SECRET")
            .unwrap_or_else(|_| panic!("LINE_CHANNEL_SECRET must be set!"));
        let http_request_body_vec = serde_json::to_vec(&request).unwrap();
        let http_request_body = http_request_body_vec.as_slice();
        // Compute the expected signature using the test channel secret and request body
        let mut mac = Hmac::<Sha256>::new_from_slice(channel_secret.as_bytes()).unwrap();
        mac.update(http_request_body);
        let expected_signature = mac.finalize().into_bytes();
        let expected_signature_str = general_purpose::STANDARD.encode(expected_signature);
        /*
         * signatureの検証に成功するテスト用のリクエストを作成する
         */
        let response = test_server
            .post("/linebot-webhook")
            .add_header(
                HeaderName::from_lowercase(b"x-line-signature").unwrap(),
                HeaderValue::from_str(&expected_signature_str).unwrap(),
            )
            .json(&request)
            .await;
        response.assert_status_ok();
        /*
         * signatureの検証エラーで失敗するテスト用のリクエストを作成する
         */
        let response = test_server
            .post("/linebot-webhook")
            .add_header(
                HeaderName::from_lowercase(b"x-line-signature").unwrap(),
                HeaderValue::from_str("invalid_signature").unwrap(),
            )
            .json(&request)
            .await;
        response.assert_status_unauthorized();
    }
}
