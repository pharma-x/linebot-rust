use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use strum_macros::Display;

use domain::model::{
    event::{Event, NewEvent, NewMessage, NewMessageEvent},
    primary_user_id::PrimaryUserId,
    talk_room::{NewTalkRoom, TalkRoom},
};

use super::event::EventTable;

#[derive(FromRow)]
pub struct TalkRoomDbTable {
    pub primary_user_id: i64,
    pub talk_room_id: String,
    pub created_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize, Clone)]
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
#[serde(tag = "type")]
pub enum LatestMessageTable {
    Follow(TalkRoomFollowTable),
    Unfollow(TalkRoomUnfollowTable),
    Postback(TalkRoomPostbackTable),
    VideoPlayComplete(TalkRoomVideoPlayCompleteTable),
    Message(TalkRoomMessageTable),
}

impl LatestMessageTable {
    pub fn document_id(&self) -> &String {
        match self {
            LatestMessageTable::Follow(e) => &e.document_id,
            LatestMessageTable::Unfollow(e) => &e.document_id,
            LatestMessageTable::Postback(e) => &e.document_id,
            LatestMessageTable::VideoPlayComplete(e) => &e.document_id,
            LatestMessageTable::Message(e) => e.document_id(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TalkRoomFollowTable {
    document_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TalkRoomUnfollowTable {
    document_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TalkRoomPostbackTable {
    document_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TalkRoomVideoPlayCompleteTable {
    document_id: String,
}

#[derive(Serialize, Deserialize, Display, Clone)]
#[serde(tag = "messageType")] // JSONにmessageTypeというフィールドでタグ名を含む
pub enum TalkRoomMessageTable {
    #[strum(serialize = "text")]
    Text(TalkRoomTextMessageTable),
    #[strum(serialize = "image")]
    Image(TalkRoomImageMessageTable),
    #[strum(serialize = "video")]
    Video(TalkRoomVideoMessageTable),
    #[strum(serialize = "audio")]
    Audio(TalkRoomAudioMessageTable),
    #[strum(serialize = "file")]
    File(TalkRoomFileMessageTable),
    #[strum(serialize = "location")]
    Location(TalkRoomLocationMessageTable),
    #[strum(serialize = "sticker")]
    Sticker(TalkRoomStickerMessageTable),
}

impl TalkRoomMessageTable {
    pub fn document_id(&self) -> &String {
        match self {
            TalkRoomMessageTable::Text(e) => &e.document_id,
            TalkRoomMessageTable::Image(e) => &e.document_id,
            TalkRoomMessageTable::Video(e) => &e.document_id,
            TalkRoomMessageTable::Audio(e) => &e.document_id,
            TalkRoomMessageTable::File(e) => &e.document_id,
            TalkRoomMessageTable::Location(e) => &e.document_id,
            TalkRoomMessageTable::Sticker(e) => &e.document_id,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TalkRoomTextMessageTable {
    document_id: String,
    text: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TalkRoomImageMessageTable {
    document_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TalkRoomVideoMessageTable {
    document_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TalkRoomAudioMessageTable {
    document_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TalkRoomFileMessageTable {
    document_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TalkRoomLocationMessageTable {
    document_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TalkRoomStickerMessageTable {
    document_id: String,
}

impl From<NewTalkRoom> for TalkRoomTable {
    fn from(s: NewTalkRoom) -> Self {
        TalkRoomTable {
            document_id: s.id.value.to_string(),
            primary_user_id: s.primary_user_id.value().to_string(),
            created_at: s.created_at,
        }
    }
}

impl From<NewTalkRoom> for TalkRoomCardTable {
    fn from(s: NewTalkRoom) -> Self {
        TalkRoomCardTable {
            document_id: s.id.value.to_string(),
            display_name: s.display_name,
            rsvp: s.rsvp,
            pinned: s.pinned,
            follow: s.follow,
            latest_message: match s.latest_message {
                NewEvent::Follow(e) => LatestMessageTable::Follow(TalkRoomFollowTable {
                    document_id: e.id.value.to_string(),
                }),
                NewEvent::Unfollow(e) => LatestMessageTable::Unfollow(TalkRoomUnfollowTable {
                    document_id: e.id.value.to_string(),
                }),
                NewEvent::Postback(e) => LatestMessageTable::Postback(TalkRoomPostbackTable {
                    document_id: e.id.value.to_string(),
                }),
                NewEvent::VideoPlayComplete(e) => {
                    LatestMessageTable::VideoPlayComplete(TalkRoomVideoPlayCompleteTable {
                        document_id: e.id.value.to_string(),
                    })
                }
                NewEvent::Message(e) => LatestMessageTable::Message(e.into()),
            },
            latest_messaged_at: s.latest_messaged_at,
            sort_time: s.sort_time,
            created_at: s.created_at,
            updated_at: s.created_at,
        }
    }
}

pub struct TalkRoomWrapper(pub TalkRoom);

impl From<(TalkRoomTable, TalkRoomCardTable, EventTable)> for TalkRoomWrapper {
    fn from(s: (TalkRoomTable, TalkRoomCardTable, EventTable)) -> Self {
        let talk_room_table = s.0;
        let talk_room_card_table = s.1;
        let event_table = s.2;

        TalkRoomWrapper(TalkRoom::new(
            talk_room_table.document_id.to_string().try_into().unwrap(),
            PrimaryUserId::new(talk_room_table.primary_user_id),
            talk_room_card_table.display_name,
            talk_room_card_table.rsvp,
            talk_room_card_table.pinned,
            talk_room_card_table.follow,
            Event::from(event_table),
            talk_room_card_table.latest_messaged_at,
            talk_room_card_table.sort_time,
            talk_room_card_table.created_at,
            talk_room_card_table.updated_at,
        ))
    }
}

impl From<NewMessageEvent> for TalkRoomMessageTable {
    fn from(s: NewMessageEvent) -> Self {
        let document_id = s.id.value.to_string();
        let message = s.message;
        match message {
            NewMessage::Text(m) => TalkRoomMessageTable::Text(TalkRoomTextMessageTable {
                document_id,
                text: m.text,
            }),
            NewMessage::Image(_) => {
                TalkRoomMessageTable::Image(TalkRoomImageMessageTable { document_id })
            }
            NewMessage::Video(_) => {
                TalkRoomMessageTable::Video(TalkRoomVideoMessageTable { document_id })
            }
            NewMessage::Audio(_) => {
                TalkRoomMessageTable::Audio(TalkRoomAudioMessageTable { document_id })
            }
            NewMessage::File(_) => {
                TalkRoomMessageTable::File(TalkRoomFileMessageTable { document_id })
            }
            NewMessage::Location(_) => {
                TalkRoomMessageTable::Location(TalkRoomLocationMessageTable { document_id })
            }
            NewMessage::Sticker(_) => {
                TalkRoomMessageTable::Sticker(TalkRoomStickerMessageTable { document_id })
            }
        }
    }
}
