use crate::context::validate::ValidatedRequest;
use crate::model::line_webhook::{LineWebhookEvent, LineWebhookRequest, LineWebhookRequests};
use crate::module::{Modules, ModulesExt};
use axum::{extract::Extension, http::StatusCode, response::IntoResponse};
use std::sync::Arc;
use tracing::error;

/*
Jsonを受け取るときは、引数の順番に気をつける必要がある
https://github.com/tokio-rs/axum/discussions/1755
https://docs.rs/axum/latest/axum/extract/index.html#the-order-of-extractors
*/
#[tracing::instrument(skip(modules))]
pub async fn line_webhook_handler(
    Extension(modules): Extension<Arc<Modules>>,
    ValidatedRequest(payload): ValidatedRequest<LineWebhookRequests>,
) -> Result<impl IntoResponse, StatusCode> {
    let requests: Vec<LineWebhookRequest> = payload.into();
    let mut result = Ok(StatusCode::OK);

    for request in requests {
        let event = &request.event;
        match event {
            LineWebhookEvent::Follow(_) => {
                result = modules
                    .linebot_webhook_usecase()
                    .create_user(request.into())
                    .await
                    .map(|_| StatusCode::OK)
                    .map_err(|err| {
                        error!("Unexpected error: {:?}", err);
                        StatusCode::INTERNAL_SERVER_ERROR
                    });
            }
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

    result
}
