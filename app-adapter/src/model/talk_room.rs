use chrono::{DateTime, Local};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use strum_macros::Display;

use domain::model::{
    event::{Event, NewEvent, NewMessage, NewMessageEvent},
    primary_user_id::PrimaryUserId,
    talk_room::{NewTalkRoom, TalkRoom},
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
    pub document_id: String,
    pub primary_user_id: String,
    #[serde(with = "firestore::serialize_as_timestamp")]
    pub created_at: DateTime<Local>,
}

// firestoreのfieldからはdocument_idを取り除く
// impl Serialize for TalkRoomTable {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::ser::Serializer,
//     {
//         let mut state = serializer.serialize_struct("TalkRoomTable", 2)?;
//         state.serialize_field("primaryUserId", &self.primary_user_id)?;
//         state.serialize_field("createdAt", &self.created_at)?;
//         state.end()
//     }
// }

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TalkRoomCardTable {
    pub document_id: String,
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

// firestoreのfieldからはdocument_idを取り除く
// impl Serialize for TalkRoomCardTable {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::ser::Serializer,
//     {
//         let mut state = serializer.serialize_struct("TalkRoomCardTable", 9)?; // 9 is the number of fields excluding document_id
//         state.serialize_field("displayName", &self.display_name)?;
//         state.serialize_field("rsvp", &self.rsvp)?;
//         state.serialize_field("pinned", &self.pinned)?;
//         state.serialize_field("follow", &self.follow)?;
//         state.serialize_field("latestMessage", &self.latest_message)?;
//         state.serialize_field("latestMessagedAt", &self.latest_messaged_at)?;
//         state.serialize_field("sortTime", &self.sort_time)?;
//         state.serialize_field("createdAt", &self.created_at)?;
//         state.serialize_field("updatedAt", &self.updated_at)?;
//         state.end()
//     }
// }

#[derive(Serialize, Deserialize, Display, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TalkRoomFollowTable {
    document_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TalkRoomUnfollowTable {
    document_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TalkRoomPostbackTable {
    document_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TalkRoomVideoPlayCompleteTable {
    document_id: String,
}

#[derive(Serialize, Deserialize, Display, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TalkRoomTextMessageTable {
    document_id: String,
    text: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TalkRoomImageMessageTable {
    document_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TalkRoomVideoMessageTable {
    document_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TalkRoomAudioMessageTable {
    document_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TalkRoomFileMessageTable {
    document_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TalkRoomLocationMessageTable {
    document_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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

impl From<(TalkRoomTable, TalkRoomCardTable, Event)> for TalkRoomWrapper {
    fn from(s: (TalkRoomTable, TalkRoomCardTable, Event)) -> Self {
        let talk_room_table = s.0;
        let talk_room_card_table = s.1;
        let event = s.2;

        TalkRoomWrapper(TalkRoom::new(
            talk_room_table.document_id.to_string().try_into().unwrap(),
            PrimaryUserId::new(talk_room_table.primary_user_id),
            talk_room_card_table.display_name,
            talk_room_card_table.rsvp,
            talk_room_card_table.pinned,
            talk_room_card_table.follow,
            event,
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
