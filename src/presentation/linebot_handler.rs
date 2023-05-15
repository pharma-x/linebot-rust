use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

pub async fn exec(Json(payload): Json<LineBotRequest>) -> impl IntoResponse {
    let events = payload.events;

    for event in events {
        println!("event: {:?}", event)
    }

    StatusCode::OK
}

#[derive(Deserialize)]
pub struct LineBotRequest {
    destination: String,
    events: Vec<LineBotEvent>,
}

#[derive(Deserialize, Debug)]
struct LineBotEvent {
    r#type: LineBotEventType, // 限られた値に制限したい
    message: Option<LineBotMessage>,
    postback: Option<LineBotPostback>,
    timestamp: u64,
    source: LineBotSource,
    reply_token: Option<String>,
    mode: String,
    webhook_event_id: String,
    delivery_context: LineDeliveryContext,
}

#[derive(Deserialize, Debug)]
struct LineBotSource {
    r#type: String,
    user_id: String,
}

#[derive(Deserialize, Debug)]
struct LineBotMessage {
    id: String,
    r#type: String,
    text: String,
}

#[derive(Deserialize, Debug)]
struct LineDeliveryContext {
    is_redelivery: bool,
}

#[derive(Deserialize, Debug)]
enum LineBotEventType {
    Message,
    Follow,
    Unfollow,
    Postback,
    VideoPlayComplete, // 不要か
}

#[derive(Deserialize, Debug)]
struct LineBotPostback {
    data: String,
    params: LineBotPostbackParams,
}

#[derive(Deserialize, Debug)]
enum LineBotPostbackParams {
    Datetime(LineBotPostbackDatetimeParams),
    RichMenu(LineBotPostbackRichMenuParams),
}

#[derive(Deserialize, Debug)]
struct LineBotPostbackDatetimeParams {
    datetime: String,
}

#[derive(Deserialize, Debug)]
struct LineBotPostbackRichMenuParams {
    new_rich_menu_alias_id: String,
    status: String,
}
