use crate::application::domain::models::line_webhook;
use axum::{http::StatusCode, response::IntoResponse, Json};

pub async fn handler(Json(payload): Json<line_webhook::LineWebhookRequest>) -> impl IntoResponse {
    let events = payload.get_events();

    for event in events.iter() {
        println!("event: {:?}", event)
    }

    StatusCode::OK.into_response()
}
