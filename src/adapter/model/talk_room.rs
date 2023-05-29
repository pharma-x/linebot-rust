use chrono::{DateTime, Local};
use serde::Serialize;

use super::event::{EventTable, EventTypeTable};

#[derive(Serialize)]
pub struct TalkRoomTable {
    #[serde(rename(serialize = "documentId"))]
    document_id: String,
    #[serde(rename(serialize = "primaryUserId"))]
    primary_user_id: String,
    #[serde(rename(serialize = "createdAt"))]
    created_at: String,
}

#[derive(Serialize)]
pub struct TalkRoomCardTable {
    #[serde(rename(serialize = "documentId"))]
    document_id: String,
    #[serde(rename(serialize = "primaryUserId"))]
    primary_user_id: String,
    #[serde(rename(serialize = "displayName"))]
    display_name: String,
    rsvp: bool,
    pinned: bool,
    follow: bool,
    #[serde(rename(serialize = "eventType"))]
    event_type: EventTypeTable,
    #[serde(rename(serialize = "latestMessage"))]
    latest_message: EventTable,
    #[serde(rename(serialize = "latestMessagedAt"))]
    latest_messaged_at: DateTime<Local>,
    #[serde(rename(serialize = "sortTime"))]
    sort_time: DateTime<Local>,
}
