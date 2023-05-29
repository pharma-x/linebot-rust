use chrono::{DateTime, Local};
use serde::Serialize;
use strum_macros::Display;
use validator::Validate;

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
    event_type: EventType,
    #[serde(rename(serialize = "latestMessage"))]
    latest_message: MessageContent,
    #[serde(rename(serialize = "latestMessagedAt"))]
    latest_messaged_at: DateTime<Local>,
    #[serde(rename(serialize = "sortTime"))]
    sort_time: DateTime<Local>,
}

#[derive(Serialize, Validate)]
pub struct MessageContent {
    #[serde(rename(serialize = "messageType"))]
    message_type: MessageType,
    text: String,
}

// Firestoreに保存するときは、文字列に変換する必要がある
#[derive(Serialize, Display)]
enum EventType {
    #[strum(serialize = "message")]
    Message,
    #[strum(serialize = "follow")]
    Follow,
    #[strum(serialize = "unfollow")]
    Unfollow,
    #[strum(serialize = "postback")]
    Postback,
    #[strum(serialize = "videoPlayComplete")]
    VideoPlayComplete,
}

#[derive(Serialize)]
enum MessageType {
    Text,
    Image,
    Video,
    Audio,
    File,
    Location,
    Stiker,
}
