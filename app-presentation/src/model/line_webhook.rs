use application::model::{
    event::{
        CreateAudioMessage, CreateContentProvider, CreateDeliveryContext, CreateEmoji, CreateEvent,
        CreateFileMessage, CreateFollowEvent, CreateImageMessage, CreateImageSet,
        CreateLocationMessage, CreateMessage, CreateMessageEvent, CreatePostback,
        CreatePostbackDatetimeParams, CreatePostbackEvent, CreatePostbackParams,
        CreatePostbackRichMenuParams, CreateStickerMessage, CreateStickerResourceType,
        CreateTextMessage, CreateUnfollowEvent, CreateUserEvent, CreateVideoMessage,
        CreateVideoPlayComplete, CreateVideoPlayCompleteEvent,
    },
    line_user_auth::CreateLineUserAuth,
};
use derive_new::new;
use serde::{Deserialize, Serialize};
use strum_macros::Display;
use strum_macros::EnumString;
use validator::Validate;

#[cfg(test)]
use fake::Dummy;

#[derive(new, Serialize, Deserialize, Debug, Validate, Clone)]
pub struct LineWebhookRequests {
    pub destination: String,
    pub events: Vec<LineWebhookEvent>,
}

#[derive(new, Debug, Validate, Clone)]
pub struct LineWebhookRequest {
    pub destination: String,
    pub event: LineWebhookEvent,
}

#[derive(Serialize, Deserialize, Debug, Clone, Display)]
#[cfg_attr(test, derive(Dummy))]
#[serde(tag = "type")]
pub enum LineWebhookEvent {
    #[strum(serialize = "follow")]
    Follow(LineWebhookFollowEvent),
    #[strum(serialize = "unfollow")]
    Unfollow(LineWebhookUnfollowEvent),
    #[strum(serialize = "postback")]
    Postback(LineWebhookPostbackEvent),
    #[strum(serialize = "videoPlayComplete")]
    VideoPlayComplete(LineWebhookVideoPlayCompleteEvent),
    #[strum(serialize = "message")]
    Message(LineWebhookMessageEvent),
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
#[cfg_attr(test, derive(Dummy))]
pub struct LineWebhookFollowEvent {
    #[serde(rename(deserialize = "replyToken"))]
    reply_token: String,
    mode: String,
    #[cfg_attr(test, dummy(faker = "chrono::Local::now().timestamp()"))]
    timestamp: i64,
    source: Option<LineWebhookSource>,
    #[serde(rename(deserialize = "webhookEventId"))]
    webhook_event_id: String,
    #[serde(rename(deserialize = "deliveryContext"))]
    delivery_context: LineDeliveryContext,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
#[cfg_attr(test, derive(Dummy))]
pub struct LineWebhookUnfollowEvent {
    #[serde(rename(deserialize = "replyToken"))]
    reply_token: String,
    mode: String,
    timestamp: i64,
    source: Option<LineWebhookSource>,
    #[serde(rename(deserialize = "webhookEventId"))]
    webhook_event_id: String,
    #[serde(rename(deserialize = "deliveryContext"))]
    delivery_context: LineDeliveryContext,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
#[cfg_attr(test, derive(Dummy))]
pub struct LineWebhookPostbackEvent {
    #[serde(rename(deserialize = "replyToken"))]
    reply_token: String,
    mode: String,
    timestamp: i64,
    source: Option<LineWebhookSource>,
    #[serde(rename(deserialize = "webhookEventId"))]
    webhook_event_id: String,
    #[serde(rename(deserialize = "deliveryContext"))]
    delivery_context: LineDeliveryContext,
    postback: LineWebhookPostback,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
struct LineWebhookPostback {
    data: String,
    params: Option<LineWebhookPostbackParams>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
#[serde(untagged)] // JSONにタグ名を含まない
enum LineWebhookPostbackParams {
    Datetime(LineWebhookPostbackDatetimeParams),
    RichMenu(LineWebhookPostbackRichMenuParams),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
enum LineWebhookPostbackDatetimeParams {
    #[serde(rename(deserialize = "datetime"))]
    DateTime(String),
    #[serde(rename(deserialize = "date"))]
    Date(String),
    #[serde(rename(deserialize = "time"))]
    Time(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
struct LineWebhookPostbackRichMenuParams {
    #[serde(rename(deserialize = "newRichMenuAliasId"))]
    new_rich_menu_alias_id: String,
    status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
#[cfg_attr(test, derive(Dummy))]
pub struct LineWebhookVideoPlayCompleteEvent {
    #[serde(rename(deserialize = "replyToken"))]
    reply_token: String,
    mode: String,
    timestamp: i64,
    source: Option<LineWebhookSource>,
    #[serde(rename(deserialize = "webhookEventId"))]
    webhook_event_id: String,
    #[serde(rename(deserialize = "deliveryContext"))]
    delivery_context: LineDeliveryContext,
    #[serde(rename(deserialize = "videoPlayComplete"))]
    video_play_complete: Option<LineWebhookVideoPlayComplete>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
struct LineWebhookVideoPlayComplete {
    tracking_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
#[cfg_attr(test, derive(Dummy))]
pub struct LineWebhookMessageEvent {
    #[serde(rename(deserialize = "replyToken"))]
    reply_token: String,
    mode: String,
    timestamp: i64,
    source: Option<LineWebhookSource>,
    #[serde(rename(deserialize = "webhookEventId"))]
    webhook_event_id: String,
    #[serde(rename(deserialize = "deliveryContext"))]
    delivery_context: LineDeliveryContext,
    message: LineWebhookMessage,
}

#[derive(Serialize, Deserialize, Debug, Clone, Display)]
#[cfg_attr(test, derive(Dummy))]
#[serde(tag = "type")]
pub enum LineWebhookSource {
    #[strum(serialize = "user")]
    User(LineWebhookUserSource),
    #[strum(serialize = "group")]
    Group(LineWebhookGroupSource),
    #[strum(serialize = "room")]
    Room(LineWebhookRoomSource),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
pub struct LineWebhookUserSource {
    #[serde(rename(deserialize = "userId"))]
    user_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
pub struct LineWebhookGroupSource {
    #[serde(rename(deserialize = "groupId"))]
    group_id: String,
    #[serde(rename(deserialize = "userId"))]
    user_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
pub struct LineWebhookRoomSource {
    #[serde(rename(deserialize = "roomId"))]
    room_id: String,
    #[serde(rename(deserialize = "userId"))]
    user_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
struct LineDeliveryContext {
    #[serde(rename(deserialize = "isRedelivery"))]
    is_redelivery: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
#[serde(tag = "type")] // JSONにtypeというフィールドでタグ名を含む
enum LineWebhookMessage {
    #[serde(rename(deserialize = "text"))]
    Text(LineWebhookTextMessage),
    #[serde(rename(deserialize = "image"))]
    Image(LineWebhookImageMessage),
    #[serde(rename(deserialize = "video"))]
    Video(LineWebhookVideoMessage),
    #[serde(rename(deserialize = "audio"))]
    Audio(LineWebhookAudioMessage),
    #[serde(rename(deserialize = "file"))]
    File(LineWebhookFileMessage),
    #[serde(rename(deserialize = "location"))]
    Location(LineWebhookLocationMessage),
    #[serde(rename(deserialize = "sticker"))]
    Sticker(LineWebhookStickerMessage),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
struct LineWebhookTextMessage {
    id: String,
    text: String,
    emojis: Vec<LineWebhookEmoji>,
    mention: Option<LineWebhookMention>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
struct LineWebhookEmoji {
    index: i32,
    length: i32,
    #[serde(rename(deserialize = "productId"))]
    product_id: String,
    #[serde(rename(deserialize = "emojiId"))]
    emoji_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
struct LineWebhookMention {
    mentionees: Vec<LineWebhookMentionee>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
#[serde(tag = "type")]
enum LineWebhookMentionee {
    LineWebhookUserMentionee,
    LineWebhookAllMentionee,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
struct LineWebhookUserMentionee {
    index: i32,
    length: i32,
    #[serde(rename(deserialize = "userId"))]
    user_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
struct LineWebhookAllMentionee {
    index: i32,
    length: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
struct LineWebhookImageMessage {
    id: String,
    #[serde(rename(deserialize = "contentProvider"))]
    content_provider: LineWebhookContentProvider,
    image_set: Option<LineWebhookImageSet>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
#[serde(tag = "type")]
enum LineWebhookContentProvider {
    #[serde(rename(deserialize = "line"))]
    Line,
    #[serde(rename(deserialize = "external"))]
    External {
        #[serde(rename(deserialize = "originalContentUrl"))]
        original_content_url: String,
        #[serde(rename(deserialize = "previewImageUrl"))]
        preview_image_url: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
struct LineWebhookImageSet {
    id: String,
    index: i32,
    length: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
struct LineWebhookVideoMessage {
    id: String,
    duration: i32,
    #[serde(rename(deserialize = "contentProvider"))]
    content_provider: LineWebhookContentProvider,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
struct LineWebhookAudioMessage {
    id: String,
    duration: i32,
    #[serde(rename(deserialize = "contentProvider"))]
    content_provider: LineWebhookContentProvider,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
struct LineWebhookFileMessage {
    id: String,
    #[serde(rename(deserialize = "fileName"))]
    file_name: String,
    #[serde(rename(deserialize = "fileSize"))]
    file_size: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
struct LineWebhookLocationMessage {
    id: String,
    title: String,
    address: String,
    latitude: f64,
    longitude: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
struct LineWebhookStickerMessage {
    id: String,
    #[serde(rename(deserialize = "packageId"))]
    package_id: String,
    #[serde(rename(deserialize = "stickerId"))]
    sticker_id: String,
    #[serde(rename(deserialize = "stickerResourceType"))]
    sticker_resource_type: LineWebhookStickerResourceType,
    keywords: Option<Vec<String>>,
    text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, EnumString)]
#[cfg_attr(test, derive(Dummy))]
enum LineWebhookStickerResourceType {
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

impl From<LineWebhookRequests> for Vec<LineWebhookRequest> {
    fn from(r: LineWebhookRequests) -> Self {
        r.events
            .iter()
            .map(|e| LineWebhookRequest {
                destination: r.destination.clone(),
                event: e.clone(),
            })
            .collect()
    }
}

impl From<LineWebhookRequest> for CreateUserEvent {
    fn from(r: LineWebhookRequest) -> Self {
        let event = r.event;
        let create_event = match event {
            LineWebhookEvent::Follow(s) => CreateEvent::Follow(s.into()),
            LineWebhookEvent::Unfollow(s) => CreateEvent::Unfollow(s.into()),
            LineWebhookEvent::Postback(s) => CreateEvent::Postback(s.into()),
            LineWebhookEvent::VideoPlayComplete(s) => CreateEvent::VideoPlayComplete(s.into()),
            LineWebhookEvent::Message(s) => CreateEvent::Message(s.into()),
        };
        Self {
            create_line_user_auth: CreateLineUserAuth {
                user_id: r.destination,
            },
            create_event,
        }
    }
}

impl From<LineWebhookFollowEvent> for CreateFollowEvent {
    fn from(s: LineWebhookFollowEvent) -> Self {
        Self {
            reply_token: s.reply_token,
            delivery_context: CreateDeliveryContext {
                is_redelivery: s.delivery_context.is_redelivery,
            },
            mode: s.mode,
            webhook_event_id: s.webhook_event_id,
            timestamp: s.timestamp,
        }
    }
}

impl From<LineWebhookUnfollowEvent> for CreateUnfollowEvent {
    fn from(s: LineWebhookUnfollowEvent) -> Self {
        Self {
            reply_token: s.reply_token,
            delivery_context: CreateDeliveryContext {
                is_redelivery: s.delivery_context.is_redelivery,
            },
            mode: s.mode,
            webhook_event_id: s.webhook_event_id,
            timestamp: s.timestamp,
        }
    }
}

impl From<LineWebhookPostbackEvent> for CreatePostbackEvent {
    fn from(s: LineWebhookPostbackEvent) -> Self {
        Self {
            reply_token: s.reply_token,
            delivery_context: CreateDeliveryContext {
                is_redelivery: s.delivery_context.is_redelivery,
            },
            postback: CreatePostback {
                data: s.postback.clone().data,
                params: s.postback.clone().params.unwrap().into(),
            },
            mode: s.mode,
            webhook_event_id: s.webhook_event_id,
            timestamp: s.timestamp,
        }
    }
}

impl From<LineWebhookPostbackParams> for CreatePostbackParams {
    fn from(s: LineWebhookPostbackParams) -> Self {
        match s {
            LineWebhookPostbackParams::Datetime(p) => CreatePostbackParams::Datetime(p.into()),
            LineWebhookPostbackParams::RichMenu(p) => CreatePostbackParams::RichMenu(p.into()),
        }
    }
}

impl From<LineWebhookPostbackDatetimeParams> for CreatePostbackDatetimeParams {
    fn from(s: LineWebhookPostbackDatetimeParams) -> Self {
        match s {
            LineWebhookPostbackDatetimeParams::DateTime(datetime) => {
                CreatePostbackDatetimeParams::DateTime(datetime)
            }
            LineWebhookPostbackDatetimeParams::Date(date) => {
                CreatePostbackDatetimeParams::Date(date)
            }
            LineWebhookPostbackDatetimeParams::Time(time) => {
                CreatePostbackDatetimeParams::Time(time)
            }
        }
    }
}

impl From<LineWebhookPostbackRichMenuParams> for CreatePostbackRichMenuParams {
    fn from(s: LineWebhookPostbackRichMenuParams) -> Self {
        Self {
            new_rich_menu_alias_id: s.new_rich_menu_alias_id,
            status: s.status,
        }
    }
}

impl From<LineWebhookVideoPlayCompleteEvent> for CreateVideoPlayCompleteEvent {
    fn from(s: LineWebhookVideoPlayCompleteEvent) -> Self {
        Self {
            reply_token: s.reply_token,
            delivery_context: CreateDeliveryContext {
                is_redelivery: s.delivery_context.is_redelivery,
            },
            video_play_complete: CreateVideoPlayComplete {
                tracking_id: s.video_play_complete.unwrap().tracking_id,
            },
            mode: s.mode,
            webhook_event_id: s.webhook_event_id,
            timestamp: s.timestamp,
        }
    }
}

impl From<LineWebhookMessageEvent> for CreateMessageEvent {
    fn from(s: LineWebhookMessageEvent) -> Self {
        Self {
            reply_token: s.reply_token,
            delivery_context: CreateDeliveryContext {
                is_redelivery: s.delivery_context.is_redelivery,
            },
            message: s.message.into(),
            mode: s.mode,
            webhook_event_id: s.webhook_event_id,
            timestamp: s.timestamp,
        }
    }
}

impl From<LineWebhookMessage> for CreateMessage {
    fn from(s: LineWebhookMessage) -> Self {
        match s {
            LineWebhookMessage::Text(m) => CreateMessage::Text(m.into()),
            LineWebhookMessage::Image(m) => CreateMessage::Image(m.into()),
            LineWebhookMessage::Video(m) => CreateMessage::Video(m.into()),
            LineWebhookMessage::Audio(m) => CreateMessage::Audio(m.into()),
            LineWebhookMessage::File(m) => CreateMessage::File(m.into()),
            LineWebhookMessage::Location(m) => CreateMessage::Location(m.into()),
            LineWebhookMessage::Sticker(m) => CreateMessage::Sticker(m.into()),
        }
    }
}

impl From<LineWebhookTextMessage> for CreateTextMessage {
    fn from(s: LineWebhookTextMessage) -> Self {
        Self {
            id: s.id,
            text: s.text,
            emojis: s
                .emojis
                .iter()
                .map(|e| CreateEmoji {
                    index: e.index,
                    length: e.length,
                    product_id: e.product_id.clone(),
                    emoji_id: e.emoji_id.clone(),
                })
                .collect(),
        }
    }
}

impl From<LineWebhookImageMessage> for CreateImageMessage {
    fn from(s: LineWebhookImageMessage) -> Self {
        Self {
            id: s.id,
            content_provider: s.content_provider.into(),
            image_set: s.image_set.map(|i| i.into()),
        }
    }
}

impl From<LineWebhookContentProvider> for CreateContentProvider {
    fn from(value: LineWebhookContentProvider) -> Self {
        match value {
            LineWebhookContentProvider::Line => CreateContentProvider::Line,
            LineWebhookContentProvider::External {
                original_content_url,
                preview_image_url,
            } => CreateContentProvider::External {
                original_content_url,
                preview_image_url,
            },
        }
    }
}

impl From<LineWebhookImageSet> for CreateImageSet {
    fn from(s: LineWebhookImageSet) -> Self {
        Self {
            id: s.id,
            index: s.index,
            length: s.length,
        }
    }
}

impl From<LineWebhookVideoMessage> for CreateVideoMessage {
    fn from(s: LineWebhookVideoMessage) -> Self {
        Self {
            id: s.id,
            duration: s.duration,
            content_provider: s.content_provider.into(),
        }
    }
}

impl From<LineWebhookAudioMessage> for CreateAudioMessage {
    fn from(s: LineWebhookAudioMessage) -> Self {
        Self {
            id: s.id,
            duration: s.duration,
            content_provider: s.content_provider.into(),
        }
    }
}

impl From<LineWebhookFileMessage> for CreateFileMessage {
    fn from(s: LineWebhookFileMessage) -> Self {
        Self {
            id: s.id,
            file_name: s.file_name,
            file_size: s.file_size,
        }
    }
}

impl From<LineWebhookLocationMessage> for CreateLocationMessage {
    fn from(s: LineWebhookLocationMessage) -> Self {
        Self {
            id: s.id,
            title: s.title,
            address: s.address,
            latitude: s.latitude,
            longitude: s.longitude,
        }
    }
}

impl From<LineWebhookStickerMessage> for CreateStickerMessage {
    fn from(s: LineWebhookStickerMessage) -> Self {
        Self {
            id: s.id,
            package_id: s.package_id,
            sticker_id: s.sticker_id,
            sticker_resource_type: s.sticker_resource_type.into(),
            keywords: s.keywords,
            text: s.text,
        }
    }
}

impl From<LineWebhookStickerResourceType> for CreateStickerResourceType {
    fn from(s: LineWebhookStickerResourceType) -> Self {
        match s {
            LineWebhookStickerResourceType::Static => CreateStickerResourceType::Static,
            LineWebhookStickerResourceType::Animation => CreateStickerResourceType::Animation,
            LineWebhookStickerResourceType::Sound => CreateStickerResourceType::Sound,
            LineWebhookStickerResourceType::AnimationSound => {
                CreateStickerResourceType::AnimationSound
            }
            LineWebhookStickerResourceType::Popup => CreateStickerResourceType::Popup,
            LineWebhookStickerResourceType::PupupSound => CreateStickerResourceType::PupupSound,
            LineWebhookStickerResourceType::Custom => CreateStickerResourceType::Custom,
            LineWebhookStickerResourceType::Message => CreateStickerResourceType::Message,
        }
    }
}
