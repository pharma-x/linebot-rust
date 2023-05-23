use crate::adapter::repository::line_user_auth::LineUserAuthRepository;
use crate::application::usecase::linebot_webhook_usecase::LinebotWebhookUseCase;
use crate::presentation::model::line_webhook::{LineWebhookEventType, LineWebhookRequest};
use axum::{http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;

pub async fn handler(Json(payload): Json<LineWebhookRequest>) -> impl IntoResponse {
    let events: Vec<crate::presentation::model::line_webhook::LineWebhookEvent> =
        payload.get_events();

    for event in events {
        let event_type = &event.r#type;
        match event_type {
            LineWebhookEventType::Follow => {
                let linebot_webhook_usecase =
                    LinebotWebhookUseCase::new(Arc::new(LineUserAuthRepository::new()));
                linebot_webhook_usecase.create_user(event.into()).await;
            }
            LineWebhookEventType::Unfollow => {
                println!("event_type: {:?}", event_type);
            }
            LineWebhookEventType::Message => {
                println!("event_type: {:?}", event_type);
            }
            LineWebhookEventType::Postback => {
                println!("event_type: {:?}", event_type);
            }
        }
    }

    StatusCode::OK.into_response()
}
