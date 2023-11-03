use chrono::{DateTime, Local};
use rust_decimal::{prelude::FromPrimitive, prelude::ToPrimitive, Decimal};
use serde::{Deserialize, Serialize};
use strum_macros::Display;

use domain::model::{
    event::{
        AudioMessage, ContentProvider, DeliveryContext, Emoji, Event, FileMessage, FollowEvent,
        ImageMessage, ImageSet, LocationMessage, Message, MessageEvent, NewAudioMessage,
        NewContentProvider, NewDeliveryContext, NewEmoji, NewEvent, NewFileMessage, NewFollowEvent,
        NewImageMessage, NewImageSet, NewLocationMessage, NewMessage, NewMessageEvent,
        NewPostbackDatetimeParams, NewPostbackEvent, NewPostbackParams, NewPostbackRichMenuParams,
        NewStickerMessage, NewStickerResourceType, NewTextMessage, NewUnfollowEvent,
        NewVideoMessage, NewVideoPlayCompleteEvent, Postback, PostbackDatetimeParams,
        PostbackEvent, PostbackParams, PostbackRichMenuParams, StickerMessage, StickerResourceType,
        TextMessage, UnfollowEvent, VideoMessage, VideoPlayComplete, VideoPlayCompleteEvent,
    },
    Id,
};

#[derive(Serialize, Deserialize, Display, Clone)]
#[serde(tag = "type")]
pub enum EventTable {
    Follow(FollowEventTable),
    Unfollow(UnfollowEventTable),
    Postback(PostbackEventTable),
    VideoPlayComplete(VideoPlayCompleteEventTable),
    Message(MessageEventTable),
}

impl EventTable {
    pub fn document_id(&self) -> &String {
        match self {
            EventTable::Follow(e) => &e.document_id,
            EventTable::Unfollow(e) => &e.document_id,
            EventTable::Message(e) => &e.document_id,
            EventTable::Postback(e) => &e.document_id,
            EventTable::VideoPlayComplete(e) => &e.document_id,
        }
    }
    pub fn created_at(&self) -> DateTime<Local> {
        match self {
            EventTable::Follow(e) => e.created_at,
            EventTable::Unfollow(e) => e.created_at,
            EventTable::Message(e) => e.created_at,
            EventTable::Postback(e) => e.created_at,
            EventTable::VideoPlayComplete(e) => e.created_at,
        }
    }
}

impl From<EventTable> for Event {
    fn from(s: EventTable) -> Self {
        match s {
            EventTable::Follow(f) => Event::Follow(f.into()),
            EventTable::Unfollow(u) => Event::Unfollow(u.into()),
            EventTable::Message(m) => Event::Message(m.into()),
            EventTable::Postback(p) => Event::Postback(p.into()),
            EventTable::VideoPlayComplete(v) => Event::VideoPlayComplete(v.into()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FollowEventTable {
    #[serde(rename(serialize = "documentId"))]
    document_id: String,
    #[serde(rename(serialize = "replyToken"))]
    reply_token: String,
    #[serde(rename(serialize = "webhookEventId"))]
    webhook_event_id: String,
    #[serde(rename(serialize = "DeliveryContext"))]
    delivery_context: DeliveryContextTable,
    mode: String,
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

impl From<FollowEventTable> for FollowEvent {
    fn from(e: FollowEventTable) -> Self {
        FollowEvent {
            id: Id::try_from(e.document_id).unwrap(),
            reply_token: e.reply_token,
            delivery_context: DeliveryContext::from(e.delivery_context),
            mode: e.mode,
            webhook_event_id: e.webhook_event_id,
            created_at: e.created_at,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UnfollowEventTable {
    #[serde(rename(serialize = "documentId"))]
    document_id: String,
    #[serde(rename(serialize = "webhookEventId"))]
    webhook_event_id: String,
    #[serde(rename(serialize = "DeliveryContext"))]
    delivery_context: DeliveryContextTable,
    mode: String,
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

impl From<UnfollowEventTable> for UnfollowEvent {
    fn from(e: UnfollowEventTable) -> Self {
        UnfollowEvent {
            id: Id::try_from(e.document_id).unwrap(),
            delivery_context: DeliveryContext::from(e.delivery_context),
            mode: e.mode,
            webhook_event_id: e.webhook_event_id,
            created_at: e.created_at,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MessageEventTable {
    #[serde(rename(serialize = "documentId"))]
    document_id: String,
    #[serde(rename(serialize = "replyToken"))]
    reply_token: String,
    #[serde(rename(serialize = "webhookEventId"))]
    webhook_event_id: String,
    #[serde(rename(serialize = "DeliveryContext"))]
    delivery_context: DeliveryContextTable,
    mode: String,
    #[serde(rename(serialize = "communicationType"))]
    communication_type: CommunicationTypeTable,
    #[serde(rename(serialize = "sendingType"))]
    sending_type: SendingTypeTable,
    #[serde(rename(serialize = "sendingMethod"))]
    sending_method: SendingMethod,
    pub message: MessageTable,
    #[serde(rename(serialize = "createdAt"))]
    created_at: DateTime<Local>,
    #[serde(rename(serialize = "updatedAt"))]
    updated_at: DateTime<Local>,
}

impl From<MessageEventTable> for MessageEvent {
    fn from(e: MessageEventTable) -> Self {
        MessageEvent {
            id: Id::try_from(e.document_id).unwrap(),
            reply_token: e.reply_token,
            delivery_context: DeliveryContext::from(e.delivery_context),
            mode: e.mode,
            webhook_event_id: e.webhook_event_id,
            message: e.message.into(),
            created_at: e.created_at,
        }
    }
}

impl From<MessageTable> for Message {
    fn from(m: MessageTable) -> Self {
        match m {
            MessageTable::Text(t) => Message::Text(t.into()),
            MessageTable::Image(i) => Message::Image(i.into()),
            MessageTable::Video(v) => Message::Video(v.into()),
            MessageTable::Audio(a) => Message::Audio(a.into()),
            MessageTable::File(f) => Message::File(f.into()),
            MessageTable::Location(l) => Message::Location(l.into()),
            MessageTable::Sticker(s) => Message::Sticker(s.into()),
        }
    }
}

impl From<TextMessageTable> for TextMessage {
    fn from(m: TextMessageTable) -> Self {
        Self {
            id: m.id,
            text: m.text,
            emojis: m.emojis.into_iter().map(|e| e.into()).collect(),
        }
    }
}

impl From<EmojiTable> for Emoji {
    fn from(e: EmojiTable) -> Self {
        Self {
            index: e.index,
            length: e.length,
            product_id: e.product_id,
            emoji_id: e.emoji_id,
        }
    }
}

impl From<ImageMessageTable> for ImageMessage {
    fn from(m: ImageMessageTable) -> Self {
        Self {
            id: m.id,
            content_provider: m.content_provider.into(),
            image_set: m.image_set.map(|i| i.into()),
        }
    }
}

impl From<ImageSetTable> for ImageSet {
    fn from(i: ImageSetTable) -> Self {
        Self {
            id: i.id,
            index: i.index,
            length: i.length,
        }
    }
}

impl From<VideoMessageTable> for VideoMessage {
    fn from(m: VideoMessageTable) -> Self {
        Self {
            id: m.id,
            duration: m.duration,
            content_provider: m.content_provider.into(),
        }
    }
}

impl From<AudioMessageTable> for AudioMessage {
    fn from(m: AudioMessageTable) -> Self {
        Self {
            id: m.id,
            duration: m.duration,
            content_provider: m.content_provider.into(),
        }
    }
}

impl From<FileMessageTable> for FileMessage {
    fn from(m: FileMessageTable) -> Self {
        Self {
            id: m.id,
            file_name: m.file_name,
            file_size: m.file_size,
        }
    }
}

impl From<LocationMessageTable> for LocationMessage {
    fn from(m: LocationMessageTable) -> Self {
        Self {
            id: m.id,
            title: m.title,
            address: m.address,
            latitude: Decimal::from_f64(m.latitude)
                .unwrap_or_else(|| panic!("Failed to convert f64 {} to Decimal", m.latitude)),
            longitude: Decimal::from_f64(m.latitude)
                .unwrap_or_else(|| panic!("Failed to convert f64 {} to Decimal", m.latitude)),
        }
    }
}

impl From<StickerMessageTable> for StickerMessage {
    fn from(m: StickerMessageTable) -> Self {
        Self {
            id: m.id,
            package_id: m.package_id,
            sticker_id: m.sticker_id,
            sticker_resource_type: m.sticker_resource_type.into(),
            keywords: m.keywords,
            text: m.text,
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

#[derive(Serialize, Deserialize, Clone)]
pub struct PostbackEventTable {
    #[serde(rename(serialize = "documentId"))]
    document_id: String,
    #[serde(rename(serialize = "replyToken"))]
    reply_token: String,
    #[serde(rename(serialize = "webhookEventId"))]
    webhook_event_id: String,
    #[serde(rename(serialize = "DeliveryContext"))]
    delivery_context: DeliveryContextTable,
    mode: String,
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

#[derive(Serialize, Deserialize, Clone)]
pub struct DeliveryContextTable {
    is_redelivery: bool,
}

impl From<DeliveryContextTable> for DeliveryContext {
    fn from(d: DeliveryContextTable) -> Self {
        Self {
            is_redelivery: d.is_redelivery,
        }
    }
}

impl From<PostbackEventTable> for PostbackEvent {
    fn from(e: PostbackEventTable) -> Self {
        Self {
            id: Id::try_from(e.document_id).unwrap(),
            reply_token: e.reply_token,
            delivery_context: DeliveryContext::from(e.delivery_context),
            mode: e.mode,
            webhook_event_id: e.webhook_event_id,
            postback: e.postback.into(),
            created_at: e.created_at,
        }
    }
}

impl From<PostbackTable> for Postback {
    fn from(p: PostbackTable) -> Self {
        Self {
            data: p.data,
            params: match p.params {
                PostbackParamsTable::Datetime(p) => PostbackParams::Datetime(p.into()),
                PostbackParamsTable::RichMenu(p) => PostbackParams::RichMenu(p.into()),
            },
        }
    }
}

impl From<PostbackDatetimeParamsTable> for PostbackDatetimeParams {
    fn from(p: PostbackDatetimeParamsTable) -> Self {
        match p {
            PostbackDatetimeParamsTable::DateTime(d) => PostbackDatetimeParams::DateTime(d),
            PostbackDatetimeParamsTable::Date(d) => PostbackDatetimeParams::Date(d),
            PostbackDatetimeParamsTable::Time(t) => PostbackDatetimeParams::Time(t),
        }
    }
}

impl From<PostbackRichMenuParamsTable> for PostbackRichMenuParams {
    fn from(p: PostbackRichMenuParamsTable) -> Self {
        Self {
            new_rich_menu_alias_id: p.new_rich_menu_alias_id,
            status: p.status,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct VideoPlayCompleteEventTable {
    #[serde(rename(serialize = "documentId"))]
    document_id: String,
    #[serde(rename(serialize = "replyToken"))]
    reply_token: String,
    #[serde(rename(serialize = "webhookEventId"))]
    webhook_event_id: String,
    #[serde(rename(serialize = "DeliveryContext"))]
    delivery_context: DeliveryContextTable,
    mode: String,
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

impl From<VideoPlayCompleteEventTable> for VideoPlayCompleteEvent {
    fn from(e: VideoPlayCompleteEventTable) -> Self {
        Self {
            id: Id::try_from(e.document_id).unwrap(),
            reply_token: e.reply_token,
            delivery_context: DeliveryContext::from(e.delivery_context),
            mode: e.mode,
            webhook_event_id: e.webhook_event_id,
            video_play_complete: e.video_play_complete.into(),
            created_at: e.created_at,
        }
    }
}

impl From<VideoPlayCompleteTable> for VideoPlayComplete {
    fn from(v: VideoPlayCompleteTable) -> Self {
        Self {
            tracking_id: v.tracking_id,
        }
    }
}

#[derive(Serialize, Deserialize, Display, Clone)]
pub enum CommunicationTypeTable {
    #[strum(serialize = "send")]
    Send,
    #[strum(serialize = "receive")]
    Receive,
}

#[derive(Serialize, Deserialize, Display, Clone)]
pub enum SendingTypeTable {
    #[strum(serialize = "manual")]
    Manual,
    #[strum(serialize = "bot")]
    Bot,
}

#[derive(Serialize, Deserialize, Display, Clone)]
pub enum SendingMethod {
    #[strum(serialize = "replay")]
    Reply,
    #[strum(serialize = "push")]
    Push,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Sender {
    id: i64,
    name: String,
    picture_url: String,
    email: String,
    sender_role: SenderRoleTable,
}

#[derive(Serialize, Deserialize, Display, Clone)]
enum SenderRoleTable {
    #[strum(serialize = "sender")]
    Sender,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PostbackTable {
    pub data: String,
    pub params: PostbackParamsTable,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum PostbackParamsTable {
    Datetime(PostbackDatetimeParamsTable),
    RichMenu(PostbackRichMenuParamsTable),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum PostbackDatetimeParamsTable {
    #[serde(rename(serialize = "datetime"))]
    DateTime(String),
    #[serde(rename(serialize = "date"))]
    Date(String),
    #[serde(rename(serialize = "time"))]
    Time(String),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PostbackRichMenuParamsTable {
    #[serde(rename(serialize = "newRichMenuAliasId"))]
    pub new_rich_menu_alias_id: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Clone)]
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
    pub image_set: Option<ImageSetTable>,
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
    pub id: String,
    pub title: String,
    pub address: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StickerMessageTable {
    pub id: String,
    #[serde(rename(serialize = "packageId"))]
    pub package_id: String,
    #[serde(rename(serialize = "stickerId"))]
    pub sticker_id: String,
    #[serde(rename(serialize = "stickerResourceType"))]
    pub sticker_resource_type: StickerResourceTypeTable,
    pub keywords: Option<Vec<String>>,
    pub text: Option<String>,
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

impl From<NewEvent> for EventTable {
    fn from(e: NewEvent) -> Self {
        match e {
            NewEvent::Follow(f) => EventTable::Follow(f.into()),
            NewEvent::Unfollow(u) => EventTable::Unfollow(u.into()),
            NewEvent::Message(m) => EventTable::Message(m.into()),
            NewEvent::Postback(p) => EventTable::Postback(p.into()),
            NewEvent::VideoPlayComplete(v) => EventTable::VideoPlayComplete(v.into()),
        }
    }
}

impl From<NewFollowEvent> for FollowEventTable {
    fn from(e: NewFollowEvent) -> Self {
        FollowEventTable {
            document_id: e.id.value.to_string(),
            reply_token: e.reply_token,
            webhook_event_id: e.webhook_event_id,
            delivery_context: e.delivery_context.into(),
            mode: e.mode,
            communication_type: CommunicationTypeTable::Receive,
            sending_type: SendingTypeTable::Bot,
            sending_method: SendingMethod::Reply,
            created_at: e.created_at,
            updated_at: e.created_at,
        }
    }
}

impl From<NewUnfollowEvent> for UnfollowEventTable {
    fn from(e: NewUnfollowEvent) -> Self {
        UnfollowEventTable {
            document_id: e.id.value.to_string(),
            webhook_event_id: e.webhook_event_id,
            delivery_context: e.delivery_context.into(),
            mode: e.mode,
            communication_type: CommunicationTypeTable::Receive,
            sending_type: SendingTypeTable::Bot,
            sending_method: SendingMethod::Reply,
            created_at: e.created_at,
            updated_at: e.created_at,
        }
    }
}

impl From<NewMessageEvent> for MessageEventTable {
    fn from(e: NewMessageEvent) -> Self {
        MessageEventTable {
            document_id: e.id.value.to_string(),
            reply_token: e.reply_token,
            webhook_event_id: e.webhook_event_id,
            delivery_context: e.delivery_context.into(),
            mode: e.mode,
            communication_type: CommunicationTypeTable::Receive,
            sending_type: SendingTypeTable::Bot,
            sending_method: SendingMethod::Reply,
            message: e.message.into(),
            created_at: e.created_at,
            updated_at: e.created_at,
        }
    }
}

impl From<NewPostbackEvent> for PostbackEventTable {
    fn from(e: NewPostbackEvent) -> Self {
        PostbackEventTable {
            document_id: e.id.value.to_string(),
            reply_token: e.reply_token,
            webhook_event_id: e.webhook_event_id,
            delivery_context: e.delivery_context.into(),
            mode: e.mode,
            communication_type: CommunicationTypeTable::Receive,
            sending_type: SendingTypeTable::Bot,
            sending_method: SendingMethod::Reply,
            postback: PostbackTable {
                data: e.postback.data,
                params: match e.postback.params {
                    NewPostbackParams::Datetime(p) => PostbackParamsTable::Datetime(p.into()),
                    NewPostbackParams::RichMenu(p) => PostbackParamsTable::RichMenu(p.into()),
                },
            },
            created_at: e.created_at,
            updated_at: e.created_at,
        }
    }
}

impl From<NewDeliveryContext> for DeliveryContextTable {
    fn from(d: NewDeliveryContext) -> Self {
        DeliveryContextTable {
            is_redelivery: d.is_redelivery,
        }
    }
}

impl From<NewPostbackDatetimeParams> for PostbackDatetimeParamsTable {
    fn from(p: NewPostbackDatetimeParams) -> Self {
        match p {
            NewPostbackDatetimeParams::DateTime(p) => PostbackDatetimeParamsTable::DateTime(p),
            NewPostbackDatetimeParams::Date(p) => PostbackDatetimeParamsTable::Date(p),
            NewPostbackDatetimeParams::Time(p) => PostbackDatetimeParamsTable::Time(p),
        }
    }
}

impl From<NewPostbackRichMenuParams> for PostbackRichMenuParamsTable {
    fn from(p: NewPostbackRichMenuParams) -> Self {
        PostbackRichMenuParamsTable {
            new_rich_menu_alias_id: p.new_rich_menu_alias_id,
            status: p.status,
        }
    }
}

impl From<NewVideoPlayCompleteEvent> for VideoPlayCompleteEventTable {
    fn from(e: NewVideoPlayCompleteEvent) -> Self {
        VideoPlayCompleteEventTable {
            document_id: e.id.value.to_string(),
            reply_token: e.reply_token,
            webhook_event_id: e.webhook_event_id,
            delivery_context: DeliveryContextTable {
                is_redelivery: e.delivery_context.is_redelivery,
            },
            mode: e.mode,
            communication_type: CommunicationTypeTable::Receive,
            sending_type: SendingTypeTable::Bot,
            sending_method: SendingMethod::Reply,
            video_play_complete: VideoPlayCompleteTable {
                tracking_id: e.video_play_complete.tracking_id,
            },
            created_at: e.created_at,
            updated_at: e.created_at,
        }
    }
}

// from NewMessage to MessageTable
impl From<NewMessage> for MessageTable {
    fn from(m: NewMessage) -> Self {
        match m {
            NewMessage::Text(t) => MessageTable::Text(t.into()),
            NewMessage::Image(i) => MessageTable::Image(i.into()),
            NewMessage::Video(v) => MessageTable::Video(v.into()),
            NewMessage::Audio(a) => MessageTable::Audio(a.into()),
            NewMessage::File(f) => MessageTable::File(f.into()),
            NewMessage::Location(l) => MessageTable::Location(l.into()),
            NewMessage::Sticker(s) => MessageTable::Sticker(s.into()),
        }
    }
}

impl From<NewTextMessage> for TextMessageTable {
    fn from(t: NewTextMessage) -> Self {
        TextMessageTable {
            id: t.id,
            text: t.text,
            emojis: t.emojis.into_iter().map(|e| e.into()).collect(),
        }
    }
}

impl From<NewEmoji> for EmojiTable {
    fn from(e: NewEmoji) -> Self {
        EmojiTable {
            index: e.index,
            length: e.length,
            product_id: e.product_id,
            emoji_id: e.emoji_id,
        }
    }
}

impl From<NewImageMessage> for ImageMessageTable {
    fn from(i: NewImageMessage) -> Self {
        ImageMessageTable {
            id: i.id,
            content_provider: i.content_provider.into(),
            image_set: i.image_set.map(|i| i.into()),
        }
    }
}

impl From<NewContentProvider> for ContentProviderTable {
    fn from(value: NewContentProvider) -> Self {
        match value {
            NewContentProvider::Line => ContentProviderTable::Line,
            NewContentProvider::External {
                original_content_url,
                preview_image_url,
            } => ContentProviderTable::External {
                original_content_url,
                preview_image_url,
            },
        }
    }
}

impl From<NewImageSet> for ImageSetTable {
    fn from(i: NewImageSet) -> Self {
        ImageSetTable {
            id: i.id,
            index: i.index,
            length: i.length,
        }
    }
}

impl From<NewVideoMessage> for VideoMessageTable {
    fn from(v: NewVideoMessage) -> Self {
        VideoMessageTable {
            id: v.id,
            duration: v.duration,
            content_provider: v.content_provider.into(),
        }
    }
}

impl From<NewAudioMessage> for AudioMessageTable {
    fn from(a: NewAudioMessage) -> Self {
        AudioMessageTable {
            id: a.id,
            duration: a.duration,
            content_provider: a.content_provider.into(),
        }
    }
}

impl From<NewFileMessage> for FileMessageTable {
    fn from(f: NewFileMessage) -> Self {
        FileMessageTable {
            id: f.id,
            file_name: f.file_name,
            file_size: f.file_size,
        }
    }
}

impl From<NewLocationMessage> for LocationMessageTable {
    fn from(l: NewLocationMessage) -> Self {
        LocationMessageTable {
            id: l.id,
            title: l.title,
            address: l.address,
            latitude: l
                .latitude
                .to_f64()
                .unwrap_or_else(|| panic!("Failed to convert Decimal {} to f64", l.latitude)),
            longitude: l
                .longitude
                .to_f64()
                .unwrap_or_else(|| panic!("Failed to convert Decimal {} to f64", l.longitude)),
        }
    }
}

impl From<NewStickerMessage> for StickerMessageTable {
    fn from(s: NewStickerMessage) -> Self {
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

impl From<NewStickerResourceType> for StickerResourceTypeTable {
    fn from(s: NewStickerResourceType) -> Self {
        match s {
            NewStickerResourceType::Static => StickerResourceTypeTable::Static,
            NewStickerResourceType::Animation => StickerResourceTypeTable::Animation,
            NewStickerResourceType::Sound => StickerResourceTypeTable::Sound,
            NewStickerResourceType::AnimationSound => StickerResourceTypeTable::AnimationSound,
            NewStickerResourceType::Popup => StickerResourceTypeTable::Popup,
            NewStickerResourceType::PupupSound => StickerResourceTypeTable::PupupSound,
            NewStickerResourceType::Custom => StickerResourceTypeTable::Custom,
            NewStickerResourceType::Message => StickerResourceTypeTable::Message,
        }
    }
}
