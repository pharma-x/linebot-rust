use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use strum_macros::Display;

use crate::domain::model::{
    event::{
        AudioMessage, ContentProvider, Emoji, FileMessage, ImageMessage, ImageSet, LocationMessage,
        Message, StickerMessage, StickerResourceType, TextMessage, VideoMessage,
    },
    talk_room::{
        LatestMessage, NewLatestMessage, NewTalkRoom, UpdateLatestMessage, UpdateTalkRoom,
    },
};

use super::event::{
    AudioMessageTable, ContentProviderTable, EmojiTable, FileMessageTable, ImageMessageTable,
    ImageSetTable, LocationMessageTable, MessageTable, StickerMessageTable,
    StickerResourceTypeTable, TextMessageTable, VideoMessageTable,
};

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
#[serde(tag = "eventType")]
pub enum LatestMessageTable {
    Follow,
    Unfollow,
    Postback,
    VideoPlayComplete,
    Message(MessageTable),
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
                NewLatestMessage::Follow => LatestMessageTable::Follow,
                NewLatestMessage::Unfollow => LatestMessageTable::Unfollow,
                NewLatestMessage::Postback => LatestMessageTable::Postback,
                NewLatestMessage::VideoPlayComplete => LatestMessageTable::VideoPlayComplete,
                NewLatestMessage::Message(m) => LatestMessageTable::Message(m.into()),
            },
            latest_messaged_at: s.latest_messaged_at,
            sort_time: s.sort_time,
            created_at: s.created_at,
            updated_at: s.created_at,
        }
    }
}

impl From<UpdateTalkRoom> for TalkRoomCardTable {
    fn from(s: UpdateTalkRoom) -> Self {
        TalkRoomCardTable {
            document_id: s.id.value.to_string(),
            display_name: s.display_name,
            rsvp: s.rsvp,
            pinned: s.pinned,
            follow: s.follow,
            latest_message: match s.latest_message {
                UpdateLatestMessage::Follow => LatestMessageTable::Follow,
                UpdateLatestMessage::Unfollow => LatestMessageTable::Unfollow,
                UpdateLatestMessage::Postback => LatestMessageTable::Postback,
                UpdateLatestMessage::VideoPlayComplete => LatestMessageTable::VideoPlayComplete,
                UpdateLatestMessage::Message(m) => LatestMessageTable::Message(m.into()),
            },
            latest_messaged_at: s.latest_messaged_at,
            sort_time: s.sort_time,
            created_at: s.created_at,
            updated_at: s.updated_at,
        }
    }
}

impl From<LatestMessageTable> for LatestMessage {
    fn from(s: LatestMessageTable) -> Self {
        return match s {
            LatestMessageTable::Follow => LatestMessage::Follow,
            LatestMessageTable::Unfollow => LatestMessage::Unfollow,
            LatestMessageTable::Postback => LatestMessage::Postback,
            LatestMessageTable::VideoPlayComplete => LatestMessage::VideoPlayComplete,
            LatestMessageTable::Message(m) => LatestMessage::Message(m.into()),
        };
    }
}

impl From<MessageTable> for Message {
    fn from(s: MessageTable) -> Self {
        return match s {
            MessageTable::Text(t) => Message::Text(t.into()),
            MessageTable::Image(t) => Message::Image(t.into()),
            MessageTable::Video(t) => Message::Video(t.into()),
            MessageTable::Audio(t) => Message::Audio(t.into()),
            MessageTable::File(t) => Message::File(t.into()),
            MessageTable::Location(t) => Message::Location(t.into()),
            MessageTable::Sticker(t) => Message::Sticker(t.into()),
        };
    }
}

impl From<TextMessageTable> for TextMessage {
    fn from(s: TextMessageTable) -> Self {
        TextMessage {
            id: s.id,
            text: s.text,
            emojis: s.emojis.into_iter().map(|e| e.into()).collect(),
        }
    }
}

impl From<EmojiTable> for Emoji {
    fn from(s: EmojiTable) -> Self {
        Emoji {
            index: s.index,
            length: s.length,
            product_id: s.product_id,
            emoji_id: s.emoji_id,
        }
    }
}

impl From<ImageMessageTable> for ImageMessage {
    fn from(s: ImageMessageTable) -> Self {
        ImageMessage {
            id: s.id,
            content_provider: s.content_provider.into(),
            image_set: s.image_set.into(),
        }
    }
}

impl From<ContentProviderTable> for ContentProvider {
    fn from(s: ContentProviderTable) -> Self {
        match s {
            ContentProviderTable::Line => ContentProvider::Line,
            ContentProviderTable::External {
                original_content_url,
                preview_image_url,
            } => ContentProvider::External {
                original_content_url,
                preview_image_url,
            },
        }
    }
}

impl From<ImageSetTable> for ImageSet {
    fn from(s: ImageSetTable) -> Self {
        ImageSet {
            id: s.id,
            index: s.index,
            length: s.length,
        }
    }
}

impl From<VideoMessageTable> for VideoMessage {
    fn from(s: VideoMessageTable) -> Self {
        VideoMessage {
            id: s.id,
            duration: s.duration,
            content_provider: s.content_provider.into(),
        }
    }
}

impl From<AudioMessageTable> for AudioMessage {
    fn from(s: AudioMessageTable) -> Self {
        AudioMessage {
            id: s.id,
            duration: s.duration,
            content_provider: s.content_provider.into(),
        }
    }
}

impl From<FileMessageTable> for FileMessage {
    fn from(s: FileMessageTable) -> Self {
        FileMessage {
            id: s.id,
            file_name: s.file_name,
            file_size: s.file_size,
        }
    }
}

impl From<LocationMessageTable> for LocationMessage {
    fn from(s: LocationMessageTable) -> Self {
        LocationMessage {
            id: s.id,
            title: s.title,
            address: s.address,
            latitude: s.latitude,
            longitude: s.longitude,
        }
    }
}

impl From<StickerMessageTable> for StickerMessage {
    fn from(s: StickerMessageTable) -> Self {
        StickerMessage {
            id: s.id,
            package_id: s.package_id,
            sticker_id: s.sticker_id,
            sticker_resource_type: s.sticker_resource_type.into(),
            keywords: s.keywords,
            text: s.text,
        }
    }
}

impl From<StickerResourceTypeTable> for StickerResourceType {
    fn from(s: StickerResourceTypeTable) -> Self {
        match s {
            StickerResourceTypeTable::Static => StickerResourceType::Static,
            StickerResourceTypeTable::Animation => StickerResourceType::Animation,
            StickerResourceTypeTable::Sound => StickerResourceType::Sound,
            StickerResourceTypeTable::AnimationSound => StickerResourceType::AnimationSound,
            StickerResourceTypeTable::Popup => StickerResourceType::Popup,
            StickerResourceTypeTable::PupupSound => StickerResourceType::PupupSound,
            StickerResourceTypeTable::Custom => StickerResourceType::Custom,
            StickerResourceTypeTable::Message => StickerResourceType::Message,
        }
    }
}
