use crate::model::line_webhook::{LineWebhookEvent, LineWebhookRequest, LineWebhookRequests};
use crate::module::{Modules, ModulesExt};
use axum::{
    body::Bytes,
    extract::Extension,
    http::{header::HeaderValue, HeaderMap, StatusCode},
    response::IntoResponse,
    TypedHeader,
};
use base64::{engine::general_purpose, Engine as _};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::env;
use std::sync::Arc;
use thiserror::Error;
use tracing::error;

/*
 * Jsonを受け取るときは、引数の順番に気をつける必要がある
 * https://github.com/tokio-rs/axum/discussions/1755
 * https://docs.rs/axum/latest/axum/extract/index.html#the-order-of-extractors
*/
#[tracing::instrument(skip(modules))]
pub async fn line_webhook_handler(
    Extension(modules): Extension<Arc<Modules>>,
    headers: HeaderMap,
    body_bytes: Bytes,
) -> Result<impl IntoResponse, StatusCode> {
    // x-line-signature ヘッダーを文字列として取得します。
    let x_line_signature = headers
        .get("x_line_signature")
        .ok_or(StatusCode::BAD_REQUEST)?
        .as_bytes();
    // リクエストボディを文字列として取得します。
    // let http_request_body = String::from_utf8_lossy(&body_bytes).to_string();
    let http_request_body = body_bytes.as_ref();
    // 署名を検証します。
    if let Err(err) = verify_line_webhook_signature(http_request_body, x_line_signature) {
        error!("Error: {}", err);
        return Err(StatusCode::UNAUTHORIZED);
    }
    // ここで body_bytes からあなたの payload をパースします
    let payload: LineWebhookRequests = serde_json::from_slice(&body_bytes).map_err(|err| {
        error!("Failed to parse JSON: {}", err);
        StatusCode::BAD_REQUEST
    })?;
    let requests: Vec<LineWebhookRequest> = payload.into();

    // Immediately respond with status code 200
    tokio::spawn(process_line_events(requests, modules));

    Ok(StatusCode::OK)
}

async fn process_line_events(
    requests: Vec<LineWebhookRequest>,
    modules: Arc<Modules>,
) -> anyhow::Result<()> {
    for request in requests {
        let event = &request.event;
        match event {
            LineWebhookEvent::Follow(_) => modules
                .linebot_webhook_usecase()
                .create_user(request.into())
                .await
                .map_err(|err| anyhow::anyhow!("Unexpected error: {:?}", err))?,
            LineWebhookEvent::Unfollow(e) => {
                println!("Unfollow event: {:?}", e);
            }
            LineWebhookEvent::Message(e) => {
                println!("Message event: {:?}", e);
            }
            LineWebhookEvent::Postback(e) => {
                println!("Postback event: {:?}", e);
            }
            LineWebhookEvent::VideoPlayComplete(e) => {
                println!("Other event: {:?}", e);
            }
        }
    }
    Ok(())
}

#[derive(Debug, Error)]
enum SignatureVerificationError {
    #[error("Cannnot create instanced mac with the channel secret as the key")]
    CannotCreateMac,
    #[error("Unauthorized: Invalid signature")]
    InvalidSignature,
}

/*
 * Verify LINE webhook signature
 *
 * # Arguments
 * `channel_secret` - Channel secret string
 * `http_request_body` - HTTP request body string
 * `x_line_signature` - The 'x-line-signature' header from the HTTP request
 */
fn verify_line_webhook_signature(
    http_request_body: &[u8],
    x_line_signature: &[u8],
) -> anyhow::Result<()> {
    // チャネルシークレット（あなたのシークレットに置き換えてください）
    let channel_secret = env::var("LINE_CHANNEL_SECRET")
        .unwrap_or_else(|_| panic!("LINE_CHANNEL_SECRET must be set!"));
    // Create HMAC-SHA256 instance with the channel secret as the key
    let mut mac = Hmac::<Sha256>::new_from_slice(channel_secret.as_bytes())
        .map_err(|_| SignatureVerificationError::CannotCreateMac)?;
    // Input the HTTP request body to the HMAC
    mac.update(http_request_body);
    // Obtain the result of the HMAC as a byte array
    let result = mac.finalize().into_bytes();
    // Encode the byte array into a base64 string
    let signature = general_purpose::STANDARD.encode(result);

    if signature.as_bytes() != x_line_signature {
        error!("Signature verification failed: Computed signature does not match the provided signature");
        return Err(SignatureVerificationError::InvalidSignature.into());
    }

    Ok(())
}
