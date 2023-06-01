use chrono::{DateTime, Local, TimeZone};
use serde::{Deserialize, Serialize};
use strum_macros::Display;

use crate::domain::model::event::{
    AudioMessage, ContentProvider, Emoji, Event, FileMessage, FollowEvent, ImageMessage, ImageSet,
    LocationMessage, Message, MessageEvent, PostbackDatetimeParams, PostbackEvent, PostbackParams,
    PostbackRichMenuParams, StickerMessage, StickerResourceType, TextMessage, UnfollowEvent,
    VideoMessage, VideoPlayCompleteEvent,
};

#[derive(Serialize, Deserialize, Display)]
#[serde(tag = "type")]
pub enum EventTable {
    Follow(FllowEventTable),
    Unfollow(UnfollowEventTable),
    Message(MessageEventTable),
    Postback(PostbackEventTable),
    VideoPlayComplete(VideoPlayCompleteEventTable),
}

impl EventTable {
    pub fn document_id(&self) -> String {
        return match self {
            EventTable::Follow(e) => e.document_id,
            EventTable::Unfollow(e) => e.document_id,
            EventTable::Message(e) => e.document_id,
            EventTable::Postback(e) => e.document_id,
            EventTable::VideoPlayComplete(e) => e.document_id,
        };
    }
    pub fn talk_room_id(&self) -> String {
        return match self {
            EventTable::Follow(e) => e.talk_room_id,
            EventTable::Unfollow(e) => e.talk_room_id,
            EventTable::Message(e) => e.talk_room_id,
            EventTable::Postback(e) => e.talk_room_id,
            EventTable::VideoPlayComplete(e) => e.talk_room_id,
        };
    }
    pub fn created_at(&self) -> DateTime<Local> {
        return match self {
            EventTable::Follow(e) => e.created_at,
            EventTable::Unfollow(e) => e.created_at,
            EventTable::Message(e) => e.created_at,
            EventTable::Postback(e) => e.created_at,
            EventTable::VideoPlayComplete(e) => e.created_at,
        };
    }
}

#[derive(Serialize, Deserialize)]
pub struct FllowEventTable {
    #[serde(rename(serialize = "documentId"))]
    document_id: String,
    #[serde(rename(serialize = "talkRoomId"))]
    talk_room_id: String,
    #[serde(rename(serialize = "replyToken"))]
    reply_token: String,
    #[serde(rename(serialize = "webhookEventId"))]
    webhook_event_id: String,
    #[serde(rename(serialize = "DeliveryContext"))]
    delivery_context: DeliveryContextTable,
    #[serde(rename(serialize = "communicationType"))]
    communication_type: CommunicationTypeTable,
    #[serde(rename(serialize = "sendingType"))]
    sending_type: SendingTypeTable,
    #[serde(rename(serialize = "sendingMethod"))]
    sending_method: SendingMethod,
    #[serde(rename(serialize = "createdAt"))]
    created_at: DateTime<Local>,
    #[serde(rename(serialize = "updatedAt"))]
    updated_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize)]
pub struct UnfollowEventTable {
    #[serde(rename(serialize = "documentId"))]
    document_id: String,
    #[serde(rename(serialize = "talkRoomId"))]
    talk_room_id: String,
    #[serde(rename(serialize = "replyToken"))]
    reply_token: String,
    #[serde(rename(serialize = "webhookEventId"))]
    webhook_event_id: String,
    #[serde(rename(serialize = "DeliveryContext"))]
    delivery_context: DeliveryContextTable,
    #[serde(rename(serialize = "communicationType"))]
    communication_type: CommunicationTypeTable,
    #[serde(rename(serialize = "sendingType"))]
    sending_type: SendingTypeTable,
    #[serde(rename(serialize = "sendingMethod"))]
    sending_method: SendingMethod,
    #[serde(rename(serialize = "createdAt"))]
    created_at: DateTime<Local>,
    #[serde(rename(serialize = "updatedAt"))]
    updated_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize)]
pub struct MessageEventTable {
    #[serde(rename(serialize = "documentId"))]
    document_id: String,
    #[serde(rename(serialize = "talkRoomId"))]
    talk_room_id: String,
    #[serde(rename(serialize = "replyToken"))]
    reply_token: String,
    #[serde(rename(serialize = "webhookEventId"))]
    webhook_event_id: String,
    #[serde(rename(serialize = "DeliveryContext"))]
    delivery_context: DeliveryContextTable,
    #[serde(rename(serialize = "communicationType"))]
    communication_type: CommunicationTypeTable,
    #[serde(rename(serialize = "sendingType"))]
    sending_type: SendingTypeTable,
    #[serde(rename(serialize = "sendingMethod"))]
    sending_method: SendingMethod,
    messages: Vec<MessageTable>,
    #[serde(rename(serialize = "createdAt"))]
    created_at: DateTime<Local>,
    #[serde(rename(serialize = "updatedAt"))]
    updated_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize)]
pub struct PostbackEventTable {
    #[serde(rename(serialize = "documentId"))]
    document_id: String,
    #[serde(rename(serialize = "talkRoomId"))]
    talk_room_id: String,
    #[serde(rename(serialize = "replyToken"))]
    reply_token: String,
    #[serde(rename(serialize = "webhookEventId"))]
    webhook_event_id: String,
    #[serde(rename(serialize = "DeliveryContext"))]
    delivery_context: DeliveryContextTable,
    #[serde(rename(serialize = "communicationType"))]
    communication_type: CommunicationTypeTable,
    #[serde(rename(serialize = "sendingType"))]
    sending_type: SendingTypeTable,
    #[serde(rename(serialize = "sendingMethod"))]
    sending_method: SendingMethod,
    postback: PostbackTable,
    #[serde(rename(serialize = "createdAt"))]
    created_at: DateTime<Local>,
    #[serde(rename(serialize = "updatedAt"))]
    updated_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize)]
pub struct VideoPlayCompleteEventTable {
    #[serde(rename(serialize = "documentId"))]
    document_id: String,
    #[serde(rename(serialize = "talkRoomId"))]
    talk_room_id: String,
    #[serde(rename(serialize = "replyToken"))]
    reply_token: String,
    #[serde(rename(serialize = "webhookEventId"))]
    webhook_event_id: String,
    #[serde(rename(serialize = "DeliveryContext"))]
    delivery_context: DeliveryContextTable,
    #[serde(rename(serialize = "communicationType"))]
    communication_type: CommunicationTypeTable,
    #[serde(rename(serialize = "sendingType"))]
    sending_type: SendingTypeTable,
    #[serde(rename(serialize = "sendingMethod"))]
    sending_method: SendingMethod,
    video_play_complete: VideoPlayCompleteTable,
    #[serde(rename(serialize = "createdAt"))]
    created_at: DateTime<Local>,
    #[serde(rename(serialize = "updatedAt"))]
    updated_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize)]
pub struct DeliveryContextTable {
    is_redelivery: bool,
}

#[derive(Serialize, Deserialize, Display)]
pub enum CommunicationTypeTable {
    #[strum(serialize = "send")]
    Send,
    #[strum(serialize = "receive")]
    Receive,
}

#[derive(Serialize, Deserialize, Display)]
pub enum SendingTypeTable {
    #[strum(serialize = "manual")]
    Manual,
    #[strum(serialize = "bot")]
    Bot,
}

#[derive(Serialize, Deserialize, Display)]
pub enum SendingMethod {
    #[strum(serialize = "replay")]
    Reply,
    #[strum(serialize = "push")]
    Push,
}

#[derive(Serialize, Deserialize)]
pub struct Sender {
    id: i64,
    name: String,
    picture_url: String,
    email: String,
    sender_role: SenderRoleTable,
}

#[derive(Serialize, Deserialize, Display)]
enum SenderRoleTable {
    #[strum(serialize = "sender")]
    Sender,
}

#[derive(Serialize, Deserialize, Display)]
pub enum EventTypeTable {
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

#[derive(Serialize, Deserialize)]
pub struct PostbackTable {
    pub data: String,
    pub params: PostbackParamsTable,
}

#[derive(Serialize, Deserialize)]
pub enum PostbackParamsTable {
    Datetime(PostbackDatetimeParamsTable),
    RichMenu(PostbackRichMenuParamsTable),
}

#[derive(Serialize, Deserialize)]
pub struct PostbackDatetimeParamsTable {
    pub datetime: String,
}

#[derive(Serialize, Deserialize)]
pub struct PostbackRichMenuParamsTable {
    #[serde(rename(serialize = "newRichMenuAliasId"))]
    pub new_rich_menu_alias_id: String,
    pub status: String,
}

#[derive(Serialize, Deserialize)]
pub struct VideoPlayCompleteTable {
    #[serde(rename(serialize = "trackingId"))]
    pub tracking_id: String,
}

#[derive(Serialize, Deserialize, Display, Clone)]
#[serde(tag = "messageType")] // JSONにmessageTypeというフィールドでタグ名を含む
pub enum MessageTable {
    #[strum(serialize = "text")]
    Text(TextMessageTable),
    #[strum(serialize = "image")]
    Image(ImageMessageTable),
    #[strum(serialize = "video")]
    Video(VideoMessageTable),
    #[strum(serialize = "audio")]
    Audio(AudioMessageTable),
    #[strum(serialize = "file")]
    File(FileMessageTable),
    #[strum(serialize = "location")]
    Location(LocationMessageTable),
    #[strum(serialize = "sticker")]
    Sticker(StickerMessageTable),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TextMessageTable {
    pub id: String,
    pub text: String,
    pub emojis: Vec<EmojiTable>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EmojiTable {
    pub index: i32,
    pub length: i32,
    #[serde(rename(serialize = "productId"))]
    pub product_id: String,
    #[serde(rename(serialize = "emojiId"))]
    pub emoji_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ImageMessageTable {
    pub id: String,
    #[serde(rename(serialize = "contentProvider"))]
    pub content_provider: ContentProviderTable,
    #[serde(rename(serialize = "imageSet"))]
    pub image_set: ImageSetTable,
}

#[derive(Serialize, Deserialize, Display, Clone)]
#[serde(tag = "type")]
pub enum ContentProviderTable {
    #[strum(serialize = "line")]
    Line,
    #[strum(serialize = "external")]
    External {
        #[serde(rename(serialize = "originalContentUrl"))]
        original_content_url: String,
        #[serde(rename(serialize = "previewImageUrl"))]
        preview_image_url: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ImageSetTable {
    pub id: String,
    pub index: i32,
    pub length: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct VideoMessageTable {
    pub id: String,
    pub duration: i32,
    #[serde(rename(serialize = "contentProvider"))]
    pub content_provider: ContentProviderTable,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AudioMessageTable {
    pub id: String,
    pub duration: i32,
    #[serde(rename(serialize = "contentProvider"))]
    pub content_provider: ContentProviderTable,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FileMessageTable {
    pub id: String,
    #[serde(rename(serialize = "fileName"))]
    pub file_name: String,
    #[serde(rename(serialize = "fileSize"))]
    pub file_size: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LocationMessageTable {
    id: String,
    title: String,
    address: String,
    latitude: f64,
    longitude: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StickerMessageTable {
    id: String,
    #[serde(rename(serialize = "packageId"))]
    package_id: String,
    #[serde(rename(serialize = "stickerId"))]
    sticker_id: String,
    #[serde(rename(serialize = "stickerResourceType"))]
    sticker_resource_type: StickerResourceTypeTable,
    keywords: Option<Vec<String>>,
    text: Option<String>,
}

#[derive(Serialize, Deserialize, Display, Clone)]
pub enum StickerResourceTypeTable {
    #[strum(serialize = "STATIC")]
    Static,
    #[strum(serialize = "ANIMATION")]
    Animation,
    #[strum(serialize = "SOUND")]
    Sound,
    #[strum(serialize = "ANIMATION_SOUND")]
    AnimationSound,
    #[strum(serialize = "POPUP")]
    Popup,
    #[strum(serialize = "POPUP_SOUND")]
    PupupSound,
    #[strum(serialize = "CUSTOM")]
    Custom,
    #[strum(serialize = "MESSAGE")]
    Message,
}

impl From<Event> for EventTable {
    fn from(e: Event) -> Self {
        match e {
            Event::Follow(f) => EventTable::Follow(f.into()),
            Event::Unfollow(u) => EventTable::Unfollow(u.into()),
            Event::Message(m) => EventTable::Message(m.into()),
            Event::Postback(p) => EventTable::Postback(p.into()),
            Event::VideoPlayComplete(v) => EventTable::VideoPlayComplete(v.into()),
        }
    }
}

impl From<FollowEvent> for FllowEventTable {
    fn from(e: FollowEvent) -> Self {
        FllowEventTable {
            document_id: e.id,
            talk_room_id: e.talk_room_id,
            reply_token: e.reply_token,
            webhook_event_id: e.webhook_event_id,
            delivery_context: DeliveryContextTable {
                is_redelivery: e.delivery_context.is_redelivery,
            },
            communication_type: CommunicationTypeTable::Receive,
            sending_type: SendingTypeTable::Bot,
            sending_method: SendingMethod::Reply,
            created_at: Local.timestamp_opt(e.timestamp, 0).unwrap(),
            updated_at: Local.timestamp_opt(e.timestamp, 0).unwrap(),
        }
    }
}

impl From<UnfollowEvent> for UnfollowEventTable {
    fn from(e: UnfollowEvent) -> Self {
        UnfollowEventTable {
            document_id: e.id,
            talk_room_id: e.talk_room_id,
            reply_token: e.reply_token,
            webhook_event_id: e.webhook_event_id,
            delivery_context: DeliveryContextTable {
                is_redelivery: e.delivery_context.is_redelivery,
            },
            communication_type: CommunicationTypeTable::Receive,
            sending_type: SendingTypeTable::Bot,
            sending_method: SendingMethod::Reply,
            created_at: Local.timestamp_opt(e.timestamp, 0).unwrap(),
            updated_at: Local.timestamp_opt(e.timestamp, 0).unwrap(),
        }
    }
}

impl From<MessageEvent> for MessageEventTable {
    fn from(e: MessageEvent) -> Self {
        MessageEventTable {
            document_id: e.id,
            talk_room_id: e.talk_room_id,
            reply_token: e.reply_token,
            webhook_event_id: e.webhook_event_id,
            delivery_context: DeliveryContextTable {
                is_redelivery: e.delivery_context.is_redelivery,
            },
            communication_type: CommunicationTypeTable::Receive,
            sending_type: SendingTypeTable::Bot,
            sending_method: SendingMethod::Reply,
            messages: vec![e.message.into()],
            created_at: Local.timestamp_opt(e.timestamp, 0).unwrap(),
            updated_at: Local.timestamp_opt(e.timestamp, 0).unwrap(),
        }
    }
}

impl From<PostbackEvent> for PostbackEventTable {
    fn from(e: PostbackEvent) -> Self {
        PostbackEventTable {
            document_id: e.id,
            talk_room_id: e.talk_room_id,
            reply_token: e.reply_token,
            webhook_event_id: e.webhook_event_id,
            delivery_context: DeliveryContextTable {
                is_redelivery: e.delivery_context.is_redelivery,
            },
            communication_type: CommunicationTypeTable::Receive,
            sending_type: SendingTypeTable::Bot,
            sending_method: SendingMethod::Reply,
            postback: PostbackTable {
                data: e.postback.data,
                params: match e.postback.params {
                    PostbackParams::Datetime(p) => PostbackParamsTable::Datetime(p.into()),
                    PostbackParams::RichMenu(p) => PostbackParamsTable::RichMenu(p.into()),
                },
            },
            created_at: Local.timestamp_opt(e.timestamp, 0).unwrap(),
            updated_at: Local.timestamp_opt(e.timestamp, 0).unwrap(),
        }
    }
}

impl From<VideoPlayCompleteEvent> for VideoPlayCompleteEventTable {
    fn from(e: VideoPlayCompleteEvent) -> Self {
        VideoPlayCompleteEventTable {
            document_id: e.id,
            talk_room_id: e.talk_room_id,
            reply_token: e.reply_token,
            webhook_event_id: e.webhook_event_id,
            delivery_context: DeliveryContextTable {
                is_redelivery: e.delivery_context.is_redelivery,
            },
            communication_type: CommunicationTypeTable::Receive,
            sending_type: SendingTypeTable::Bot,
            sending_method: SendingMethod::Reply,
            video_play_complete: VideoPlayCompleteTable {
                tracking_id: e.video_play_complete.tracking_id,
            },
            created_at: Local.timestamp_opt(e.timestamp, 0).unwrap(),
            updated_at: Local.timestamp_opt(e.timestamp, 0).unwrap(),
        }
    }
}

impl From<Message> for MessageTable {
    fn from(m: Message) -> Self {
        match m {
            Message::Text(t) => MessageTable::Text(t.into()),
            Message::Image(i) => MessageTable::Image(i.into()),
            Message::Video(v) => MessageTable::Video(v.into()),
            Message::Audio(a) => MessageTable::Audio(a.into()),
            Message::File(f) => MessageTable::File(f.into()),
            Message::Location(l) => MessageTable::Location(l.into()),
            Message::Sticker(s) => MessageTable::Sticker(s.into()),
        }
    }
}

impl From<TextMessage> for TextMessageTable {
    fn from(t: TextMessage) -> Self {
        TextMessageTable {
            id: t.id,
            text: t.text,
            emojis: t.emojis.into_iter().map(|e| e.into()).collect(),
        }
    }
}

impl From<Emoji> for EmojiTable {
    fn from(e: Emoji) -> Self {
        EmojiTable {
            index: e.index,
            length: e.length,
            product_id: e.product_id,
            emoji_id: e.emoji_id,
        }
    }
}

impl From<ImageMessage> for ImageMessageTable {
    fn from(i: ImageMessage) -> Self {
        ImageMessageTable {
            id: i.id,
            content_provider: i.content_provider.into(),
            image_set: i.image_set.into(),
        }
    }
}

impl From<ContentProvider> for ContentProviderTable {
    fn from(value: ContentProvider) -> Self {
        match value {
            ContentProvider::Line => ContentProviderTable::Line,
            ContentProvider::External {
                original_content_url,
                preview_image_url,
            } => ContentProviderTable::External {
                original_content_url,
                preview_image_url,
            },
        }
    }
}

impl From<PostbackDatetimeParams> for PostbackDatetimeParamsTable {
    fn from(p: PostbackDatetimeParams) -> Self {
        PostbackDatetimeParamsTable {
            datetime: p.datetime,
        }
    }
}

impl From<PostbackRichMenuParams> for PostbackRichMenuParamsTable {
    fn from(p: PostbackRichMenuParams) -> Self {
        PostbackRichMenuParamsTable {
            new_rich_menu_alias_id: p.new_rich_menu_alias_id,
            status: p.status,
        }
    }
}

impl From<ImageSet> for ImageSetTable {
    fn from(i: ImageSet) -> Self {
        ImageSetTable {
            id: i.id,
            index: i.index,
            length: i.length,
        }
    }
}

impl From<VideoMessage> for VideoMessageTable {
    fn from(v: VideoMessage) -> Self {
        VideoMessageTable {
            id: v.id,
            duration: v.duration,
            content_provider: v.content_provider.into(),
        }
    }
}

impl From<AudioMessage> for AudioMessageTable {
    fn from(a: AudioMessage) -> Self {
        AudioMessageTable {
            id: a.id,
            duration: a.duration,
            content_provider: a.content_provider.into(),
        }
    }
}

impl From<FileMessage> for FileMessageTable {
    fn from(f: FileMessage) -> Self {
        FileMessageTable {
            id: f.id,
            file_name: f.file_name,
            file_size: f.file_size,
        }
    }
}

impl From<LocationMessage> for LocationMessageTable {
    fn from(l: LocationMessage) -> Self {
        LocationMessageTable {
            id: l.id,
            title: l.title,
            address: l.address,
            latitude: l.latitude,
            longitude: l.longitude,
        }
    }
}

impl From<StickerMessage> for StickerMessageTable {
    fn from(s: StickerMessage) -> Self {
        StickerMessageTable {
            id: s.id,
            package_id: s.package_id,
            sticker_id: s.sticker_id,
            sticker_resource_type: s.sticker_resource_type.into(),
            keywords: s.keywords,
            text: s.text,
        }
    }
}

impl From<StickerResourceType> for StickerResourceTypeTable {
    fn from(s: StickerResourceType) -> Self {
        match s {
            StickerResourceType::Static => StickerResourceTypeTable::Static,
            StickerResourceType::Animation => StickerResourceTypeTable::Animation,
            StickerResourceType::Sound => StickerResourceTypeTable::Sound,
            StickerResourceType::AnimationSound => StickerResourceTypeTable::AnimationSound,
            StickerResourceType::Popup => StickerResourceTypeTable::Popup,
            StickerResourceType::PupupSound => StickerResourceTypeTable::PupupSound,
            StickerResourceType::Custom => StickerResourceTypeTable::Custom,
            StickerResourceType::Message => StickerResourceTypeTable::Message,
        }
    }
}
