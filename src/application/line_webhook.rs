use crate::application::domain::models::line_webhook::{LineWebhookEventType, LineWebhookRequest};
use axum::{http::StatusCode, response::IntoResponse, Json};

pub async fn handler(Json(payload): Json<LineWebhookRequest>) -> impl IntoResponse {
    let events = payload.get_events();

    for event in events.iter() {
        let event_type = &event.r#type;
        match event_type {
            LineWebhookEventType::Follow => {
                println!("event_type: {:?}", event_type);
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
