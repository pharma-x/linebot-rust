use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use strum_macros::Display;

use domain::model::{
    event::{NewEvent, NewMessage, NewMessageEvent},
    send_message::NewSendMessage,
    talk_room::{NewLatestMessages, NewTalkRoom},
};

#[derive(FromRow, Debug)]
pub struct TalkRoomDbTable {
    pub document_id: String,
    pub primary_user_id: String,
    pub created_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TalkRoomTable {
    pub primary_user_id: String,
    #[serde(with = "firestore::serialize_as_timestamp")]
    pub created_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TalkRoomCardTable {
    pub display_name: String,
    pub rsvp: bool,
    pub pinned: bool,
    pub follow: bool,
    pub latest_message: LatestMessageTable,
    #[serde(with = "firestore::serialize_as_timestamp")]
    pub latest_messaged_at: DateTime<Local>,
    #[serde(with = "firestore::serialize_as_timestamp")]
    pub sort_time: DateTime<Local>,
    #[serde(with = "firestore::serialize_as_timestamp")]
    pub created_at: DateTime<Local>,
    #[serde(with = "firestore::serialize_as_timestamp")]
    pub updated_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize, Display, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TalkRoomFollowTable {
    document_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TalkRoomUnfollowTable {
    document_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TalkRoomPostbackTable {
    document_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TalkRoomVideoPlayCompleteTable {
    document_id: String,
}

#[derive(Serialize, Deserialize, Display, Clone, Debug)]
#[serde(tag = "messageType")] // JSONにmessageTypeというフィールドでタグ名を含む
#[serde(rename_all = "lowercase")]
pub enum TalkRoomMessageTable {
    Text(TalkRoomTextMessageTable),
    Image(TalkRoomImageMessageTable),
    Video(TalkRoomVideoMessageTable),
    Audio(TalkRoomAudioMessageTable),
    File(TalkRoomFileMessageTable),
    Location(TalkRoomLocationMessageTable),
    Sticker(TalkRoomStickerMessageTable),
    Imagemap(TalkRoomImagemapMessageTable),
    Template(TalkRoomTemplateMessageTable),
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
            TalkRoomMessageTable::Imagemap(e) => &e.document_id,
            TalkRoomMessageTable::Template(e) => &e.document_id,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TalkRoomTextMessageTable {
    document_id: String,
    text: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TalkRoomImageMessageTable {
    document_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TalkRoomVideoMessageTable {
    document_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TalkRoomAudioMessageTable {
    document_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TalkRoomFileMessageTable {
    document_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TalkRoomLocationMessageTable {
    document_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TalkRoomStickerMessageTable {
    document_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TalkRoomImagemapMessageTable {
    document_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TalkRoomTemplateMessageTable {
    document_id: String,
}

impl From<NewTalkRoom> for TalkRoomTable {
    fn from(s: NewTalkRoom) -> Self {
        TalkRoomTable {
            primary_user_id: s.primary_user_id.value().to_string(),
            created_at: s.created_at,
        }
    }
}

impl From<NewTalkRoom> for TalkRoomCardTable {
    fn from(s: NewTalkRoom) -> Self {
        TalkRoomCardTable {
            display_name: s.display_name,
            rsvp: s.rsvp,
            pinned: s.pinned,
            follow: s.follow,
            latest_message: s.latest_messages.into(),
            latest_messaged_at: s.latest_messaged_at,
            sort_time: s.sort_time,
            created_at: s.created_at,
            updated_at: s.created_at,
        }
    }
}

impl From<NewLatestMessages> for LatestMessageTable {
    fn from(s: NewLatestMessages) -> Self {
        match s {
            NewLatestMessages::Event(e) => e.into(),
            NewLatestMessages::SendMessages(m) => {
                LatestMessageTable::from(m.messages.last().unwrap().clone(), m.id.value.to_string())
            }
        }
    }
}

impl From<NewEvent> for LatestMessageTable {
    fn from(s: NewEvent) -> Self {
        match s {
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
        }
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

impl LatestMessageTable {
    fn from(message: NewSendMessage, document_id: String) -> Self {
        LatestMessageTable::Message(TalkRoomMessageTable::from(message, document_id))
    }
}

impl TalkRoomMessageTable {
    fn from(message: NewSendMessage, document_id: String) -> Self {
        match message {
            NewSendMessage::Text(m) => TalkRoomMessageTable::Text(TalkRoomTextMessageTable {
                document_id,
                text: m.text,
            }),
            NewSendMessage::Sticker(_) => {
                TalkRoomMessageTable::Sticker(TalkRoomStickerMessageTable { document_id })
            }
            NewSendMessage::Image(_) => {
                TalkRoomMessageTable::Image(TalkRoomImageMessageTable { document_id })
            }
            NewSendMessage::Video(_) => {
                TalkRoomMessageTable::Video(TalkRoomVideoMessageTable { document_id })
            }
            NewSendMessage::Audio(_) => {
                TalkRoomMessageTable::Audio(TalkRoomAudioMessageTable { document_id })
            }
            NewSendMessage::Location(_) => {
                TalkRoomMessageTable::Location(TalkRoomLocationMessageTable { document_id })
            }
            NewSendMessage::Imagemap(_) => {
                TalkRoomMessageTable::Imagemap(TalkRoomImagemapMessageTable { document_id })
            }
            NewSendMessage::Template(_) => {
                TalkRoomMessageTable::Template(TalkRoomTemplateMessageTable { document_id })
            }
        }
    }
}
