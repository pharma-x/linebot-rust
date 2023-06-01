use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use strum_macros::Display;

use crate::domain::model::talk_room::{LatestMessage, TalkRoom};

use super::event::MessageTable;

#[derive(Serialize, Deserialize)]
pub struct TalkRoomTable {
    #[serde(rename(serialize = "documentId"))]
    pub document_id: String,
    #[serde(rename(serialize = "primaryUserId"))]
    pub primary_user_id: String,
    #[serde(rename(serialize = "createdAt"))]
    #[serde(with = "firestore::serialize_as_timestamp")]
    pub created_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TalkRoomCardTable {
    #[serde(rename(serialize = "documentId"))]
    pub document_id: String,
    #[serde(rename(serialize = "displayName"))]
    pub display_name: String,
    pub rsvp: bool,
    pub pinned: bool,
    pub follow: bool,
    #[serde(rename(serialize = "latestMessage"))]
    pub latest_message: LatestMessageTable,
    #[serde(rename(serialize = "latestMessagedAt"))]
    #[serde(with = "firestore::serialize_as_timestamp")]
    pub latest_messaged_at: DateTime<Local>,
    #[serde(rename(serialize = "sortTime"))]
    #[serde(with = "firestore::serialize_as_timestamp")]
    pub sort_time: DateTime<Local>,
    #[serde(rename(serialize = "createdAt"))]
    #[serde(with = "firestore::serialize_as_timestamp")]
    pub created_at: DateTime<Local>,
    #[serde(rename(serialize = "updatedAt"))]
    #[serde(with = "firestore::serialize_as_timestamp")]
    pub updated_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize, Display, Clone)]
#[serde(tag = "eventType")]
pub enum LatestMessageTable {
    Follow,
    Unfollow,
    Postback,
    VideoPlayComplete,
    Message(MessageTable),
}

impl From<TalkRoom> for TalkRoomTable {
    fn from(s: TalkRoom) -> Self {
        TalkRoomTable {
            document_id: s.id,
            primary_user_id: s.primary_user_id.value().to_string(),
            created_at: Local::now(),
        }
    }
}

impl From<TalkRoom> for TalkRoomCardTable {
    fn from(s: TalkRoom) -> Self {
        TalkRoomCardTable {
            document_id: s.id,
            display_name: s.display_name,
            rsvp: s.rsvp,
            pinned: s.pinned,
            follow: s.follow,
            latest_message: match s.latest_message {
                LatestMessage::Follow => LatestMessageTable::Follow,
                LatestMessage::Unfollow => LatestMessageTable::Unfollow,
                LatestMessage::Postback => LatestMessageTable::Postback,
                LatestMessage::VideoPlayComplete => LatestMessageTable::VideoPlayComplete,
                LatestMessage::Message(m) => LatestMessageTable::Message(m.into()),
            },
            latest_messaged_at: s.latest_messaged_at,
            sort_time: s.sort_time,
            created_at: Local::now(),
            updated_at: Local::now(),
        }
    }
}
