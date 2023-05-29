use crate::domain::model::primary_user_id::PrimaryUserId;
use chrono::{DateTime, Local};
use derive_new::new;

#[derive(new)]
pub struct TalkRoom {
    id: String,
    primary_user_id: PrimaryUserId,
    display_name: String,
    rsvp: bool,
    pinned: bool,
    follow: bool,
    latest_message: MessageContent,
    latest_messaged_at: DateTime<Local>,
    sort_time: DateTime<Local>,
}

pub struct MessageContent {
    r#type: String,
    message_type: String,
    text: String,
}
