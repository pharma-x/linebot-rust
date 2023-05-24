use crate::application::model::line_user_auth::CreateLineUserAuth;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct LineWebhookRequest {
    destination: String,
    events: Vec<LineWebhookEvent>,
}

impl LineWebhookRequest {
    pub(in crate::presentation) fn get_events(&self) -> Vec<LineWebhookEvent> {
        self.events.clone()
    }
}

#[derive(Deserialize, Debug, Clone, Validate)]
pub(in crate::presentation) struct LineWebhookEvent {
    pub(in crate::presentation) r#type: LineWebhookEventType, // 限られた値に制限したい
    message: Option<LineWebhookMessage>,
    postback: Option<LineWebhookPostback>,
    timestamp: u64,
    source: LineWebhookSource,
    reply_token: Option<String>,
    mode: String,
    webhook_event_id: String,
    delivery_context: LineDeliveryContext,
}

#[derive(Deserialize, Debug, Clone)]
pub(in crate::presentation) enum LineWebhookEventType {
    Message,
    Follow,
    Unfollow,
    Postback,
}

#[derive(Deserialize, Debug, Clone)]
struct LineWebhookSource {
    r#type: String,
    user_id: String,
}

#[derive(Deserialize, Debug, Clone)]
struct LineDeliveryContext {
    is_redelivery: bool,
}

#[derive(Deserialize, Debug, Clone)]
struct LineWebhookMessage {
    id: String,
    r#type: String,
    text: String,
}

#[derive(Deserialize, Debug, Clone)]
struct LineWebhookPostback {
    data: String,
    params: LineWebhookPostbackParams,
}

#[derive(Deserialize, Debug, Clone)]
enum LineWebhookPostbackParams {
    Datetime(LineWebhookPostbackDatetimeParams),
    RichMenu(LineWebhookPostbackRichMenuParams),
}

#[derive(Deserialize, Debug, Clone)]
struct LineWebhookPostbackDatetimeParams {
    datetime: String,
}

#[derive(Deserialize, Debug, Clone)]
struct LineWebhookPostbackRichMenuParams {
    new_rich_menu_alias_id: String,
    status: String,
}

impl From<LineWebhookEvent> for CreateLineUserAuth {
    fn from(s: LineWebhookEvent) -> Self {
        CreateLineUserAuth {
            user_id: s.source.user_id,
        }
    }
}
