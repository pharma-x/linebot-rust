use crate::presentation::context::validate::ValidatedRequest;
use crate::presentation::model::line_webhook::{LineWebhookEventType, LineWebhookRequest};
use crate::presentation::module::{Modules, ModulesExt};
use axum::{extract::Extension, http::StatusCode, response::IntoResponse};
use std::sync::Arc;
use tracing::error;

#[tracing::instrument(skip(modules))]
pub async fn line_webhook_handler(
    Extension(modules): Extension<Arc<Modules>>,
    ValidatedRequest(payload): ValidatedRequest<LineWebhookRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let events = payload.get_events();
    let mut result = Ok(StatusCode::OK);

    for event in events {
        let event_type = &event.r#type;
        match event_type {
            LineWebhookEventType::Follow => {
                result = modules
                    .linebot_webhook_usecase()
                    .create_user(event.into())
                    .await
                    .map(|_| StatusCode::OK)
                    .map_err(|err| {
                        error!("Unexpected error: {:?}", err);
                        StatusCode::INTERNAL_SERVER_ERROR
                    });
            }
            LineWebhookEventType::Unfollow => {
                println!("Unfollow event: {:?}", event);
            }
            LineWebhookEventType::Message => {
                println!("Message event: {:?}", event);
            }
            LineWebhookEventType::Postback => {
                println!("Postback event: {:?}", event);
            }
            LineWebhookEventType::VideoPlayComplete => {
                println!("Other event: {:?}", event);
            }
        }
    }

    result
}
