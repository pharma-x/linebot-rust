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
    #[serde(rename(deserialize = "follow"))]
    Follow(LineWebhookFollowEvent),
    #[serde(rename(deserialize = "unfollow"))]
    Unfollow(LineWebhookUnfollowEvent),
    #[serde(rename(deserialize = "postback"))]
    Postback(LineWebhookPostbackEvent),
    #[serde(rename(deserialize = "videoPlayComplete"))]
    VideoPlayComplete(LineWebhookVideoPlayCompleteEvent),
    #[serde(rename(deserialize = "message"))]
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
    #[serde(rename(deserialize = "trackingId"))]
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
    #[serde(rename(deserialize = "user"))]
    User(LineWebhookUserSource),
    #[serde(rename(deserialize = "group"))]
    Group(LineWebhookGroupSource),
    #[serde(rename(deserialize = "room"))]
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
    #[serde(rename(deserialize = "user"))]
    LineWebhookUserMentionee,
    #[serde(rename(deserialize = "all"))]
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
    #[serde(rename(deserialize = "STATIC"))]
    Static,
    #[serde(rename(deserialize = "ANIMATION"))]
    Animation,
    #[serde(rename(deserialize = "SOUND"))]
    Sound,
    #[serde(rename(deserialize = "ANIMATION_SOUND"))]
    AnimationSound,
    #[serde(rename(deserialize = "POPUP"))]
    Popup,
    #[serde(rename(deserialize = "POPUP_SOUND"))]
    PopupSound,
    #[serde(rename(deserialize = "CUSTOM"))]
    Custom,
    #[serde(rename(deserialize = "MESSAGE"))]
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
            LineWebhookStickerResourceType::PopupSound => CreateStickerResourceType::PopupSound,
            LineWebhookStickerResourceType::Custom => CreateStickerResourceType::Custom,
            LineWebhookStickerResourceType::Message => CreateStickerResourceType::Message,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
     * follow event
     */
    #[test]
    fn test_line_webhook_follow_event() {
        let destination = "line_id".to_string();
        let json = r#"
        {
            "replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA",
            "type": "follow",
            "mode": "active",
            "timestamp": 1462629479859,
            "source": {
                "type": "user",
                "userId": "U00000000000000000000000000000000"
            },
            "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
            "deliveryContext": {
                "isRedelivery": false
            }
        }
        "#;
        let line_webhook_event: LineWebhookEvent =
            serde_json::from_str(json).expect("Failed to deserialize");
        println!("line_webhook_event{:?}", line_webhook_event);
        LineWebhookRequest::new(destination, line_webhook_event);
    }
    /*
     * unfollow event
     */
    #[test]
    fn test_line_webhook_unfollow_event() {
        let destination = "line_id".to_string();
        let json = r#"
        {
            "type": "unfollow",
            "mode": "active",
            "timestamp": 1462629479859,
            "source": {
                "type": "user",
                "userId": "U00000000000000000000000000000000"
                },
            "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
            "deliveryContext": {
                "isRedelivery": false
            }
        }
        "#;
        let line_webhook_event: LineWebhookEvent =
            serde_json::from_str(json).expect("Failed to deserialize");
        println!("line_webhook_event{:?}", line_webhook_event);
        LineWebhookRequest::new(destination, line_webhook_event);
    }
    /*
     * unfollow event
     */
    #[test]
    fn test_line_webhook_video_play_complete_event() {
        let destination = "line_id".to_string();
        let json = r#"
        {
            "replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA",
            "type": "videoPlayComplete",
            "mode": "active",
            "timestamp": 1462629479859,
            "source": {
                "type": "user",
                "userId": "U00000000000000000000000000000000"
            },
            "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
            "deliveryContext": {
                "isRedelivery": false
            },
            "videoPlayComplete": {
                "trackingId": "track-id"
            }
        }
        "#;
        let line_webhook_event: LineWebhookEvent =
            serde_json::from_str(json).expect("Failed to deserialize");
        println!("line_webhook_event{:?}", line_webhook_event);
        LineWebhookRequest::new(destination, line_webhook_event);
    }
    /*
     * message event
     */
    #[test]
    fn test_line_webhook_message_event() {
        let destination = "line_id".to_string();
        /*
         * text
         */
        let json = r#"
        {
            "replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA",
            "type": "message",
            "mode": "active",
            "timestamp": 1462629479859,
            "source": {
                "type": "user",
                "userId": "U00000000000000000000000000000000"
            },
            "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
            "deliveryContext": {
                "isRedelivery": false
            },
            "message": {
                "id": "444573844083572737",
                "type": "text",
                "quoteToken": "q3Plxr4AgKd...",
                "text": "@All @example Good Morning!! (love)",
                "emojis": [
                    {
                        "index": 29,
                        "length": 6,
                        "productId": "5ac1bfd5040ab15980c9b435",
                        "emojiId": "001"
                    }
                ],
                "mention": {
                    "mentionees": [
                        {
                            "index": 0,
                            "length": 4,
                            "type": "all"
                        },
                        {
                            "index": 5,
                            "length": 8,
                            "userId": "U49585cd0d5...",
                            "type": "user"
                        }
                    ]
                }
            }
        }
        "#;
        let line_webhook_event: LineWebhookEvent =
            serde_json::from_str(json).expect("Failed to deserialize");
        println!("line_webhook_event{:?}", line_webhook_event);
        LineWebhookRequest::new(destination.clone(), line_webhook_event);

        /*
         * image 1
         */
        let json = r#"
        {
            "type": "message",
            "message": {
                "type": "image",
                "id": "354718705033693859",
                "quoteToken": "q3Plxr4AgKd...",
                "contentProvider": {
                    "type": "line"
                },
                "imageSet": {
                    "id": "E005D41A7288F41B65593ED38FF6E9834B046AB36A37921A56BC236F13A91855",
                    "index": 1,
                    "total": 2
                }
            },
            "timestamp": 1627356924513,
            "source": {
                "type": "user",
                "userId": "U4af4980629..."
            },
            "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
            "deliveryContext": {
                "isRedelivery": false
            },
            "replyToken": "7840b71058e24a5d91f9b5726c7512c9",
            "mode": "active"
        }
        "#;
        let line_webhook_event: LineWebhookEvent =
            serde_json::from_str(json).expect("Failed to deserialize");
        println!("line_webhook_event{:?}", line_webhook_event);
        LineWebhookRequest::new(destination.clone(), line_webhook_event);
        /*
         * image 2
         * ドキュメントを元に自作
         */
        let json = r#"
        {
            "type": "message",
            "message": {
                "type": "image",
                "id": "354718705033693861",
                "quoteToken": "yHAz4Ua2wx7...",
                "contentProvider": {
                    "type": "external",
                    "originalContentUrl": "https://example.com",
                    "previewImageUrl": "https://example.com"
                },
                "imageSet": {
                    "id": "E005D41A7288F41B65593ED38FF6E9834B046AB36A37921A56BC236F13A91855",
                    "index": 2,
                    "total": 2
                }
            },
            "timestamp": 1627356924722,
            "source": {
                "type": "user",
                "userId": "U4af4980629..."
            },
            "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
            "deliveryContext": {
                "isRedelivery": false
            },
            "replyToken": "fbf94e269485410da6b7e3a5e33283e8",
            "mode": "active"
        }
        "#;
        let line_webhook_event: LineWebhookEvent =
            serde_json::from_str(json).expect("Failed to deserialize");
        println!("line_webhook_event{:?}", line_webhook_event);
        LineWebhookRequest::new(destination.clone(), line_webhook_event);
        /*
         * video
         */
        let json = r#"
        {
            "replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA",
            "type": "message",
            "mode": "active",
            "timestamp": 1462629479859,
            "source": {
                "type": "user",
                "userId": "U4af4980629..."
            },
            "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
            "deliveryContext": {
                "isRedelivery": false
            },
            "message": {
                "id": "325708",
                "type": "video",
                "quoteToken": "q3Plxr4AgKd...",
                "duration": 60000,
                "contentProvider": {
                    "type": "external",
                    "originalContentUrl": "https://example.com/original.mp4",
                    "previewImageUrl": "https://example.com/preview.jpg"
                }
            }
        }
        "#;
        let line_webhook_event: LineWebhookEvent =
            serde_json::from_str(json).expect("Failed to deserialize");
        println!("line_webhook_event{:?}", line_webhook_event);
        LineWebhookRequest::new(destination.clone(), line_webhook_event);
        /*
         * audio
         */
        let json = r#"
        {
            "replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA",
            "type": "message",
            "mode": "active",
            "timestamp": 1462629479859,
            "source": {
                "type": "user",
                "userId": "U4af4980629..."
            },
            "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
            "deliveryContext": {
                "isRedelivery": false
            },
            "message": {
                "id": "325708",
                "type": "audio",
                "duration": 60000,
                "contentProvider": {
                    "type": "line"
                }
            }
        }
        "#;
        let line_webhook_event: LineWebhookEvent =
            serde_json::from_str(json).expect("Failed to deserialize");
        println!("line_webhook_event{:?}", line_webhook_event);
        LineWebhookRequest::new(destination.clone(), line_webhook_event);
        /*
         * file
         */
        let json = r#"
        {
            "replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA",
            "type": "message",
            "mode": "active",
            "timestamp": 1462629479859,
            "source": {
                "type": "user",
                "userId": "U4af4980629..."
            },
            "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
            "deliveryContext": {
                "isRedelivery": false
            },
            "message": {
                "id": "325708",
                "type": "file",
                "fileName": "file.txt",
                "fileSize": 2138
            }
        }
        "#;
        let line_webhook_event: LineWebhookEvent =
            serde_json::from_str(json).expect("Failed to deserialize");
        println!("line_webhook_event{:?}", line_webhook_event);
        LineWebhookRequest::new(destination.clone(), line_webhook_event);
        /*
         * location
         */
        let json = r#"
        {
            "replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA",
            "type": "message",
            "mode": "active",
            "timestamp": 1462629479859,
            "source": {
                "type": "user",
                "userId": "U4af4980629..."
            },
            "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
            "deliveryContext": {
                "isRedelivery": false
            },
            "message": {
                "id": "325708",
                "type": "location",
                "title": "my location",
                "address": "日本、〒102-8282 東京都千代田区紀尾井町1番3号",
                "latitude": 35.67966,
                "longitude": 139.73669
            }
        }
        "#;
        let line_webhook_event: LineWebhookEvent =
            serde_json::from_str(json).expect("Failed to deserialize");
        println!("line_webhook_event{:?}", line_webhook_event);
        LineWebhookRequest::new(destination.clone(), line_webhook_event);
        /*
         * sticker
         * アニメーションスタンプの例
         */
        let json = r#"
        {
            "replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA",
            "type": "message",
            "mode": "active",
            "timestamp": 1462629479859,
            "source": {
                "type": "user",
                "userId": "U4af4980629..."
            },
            "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
            "deliveryContext": {
                "isRedelivery": false
            },
            "message": {
                "type": "sticker",
                "id": "1501597916",
                "quoteToken": "q3Plxr4AgKd...",
                "stickerId": "52002738",
                "packageId": "11537",
                "stickerResourceType": "ANIMATION",
                "keywords": [
                    "cony",
                    "sally",
                    "Staring",
                    "hi",
                    "whatsup",
                    "line",
                    "howdy",
                    "HEY",
                    "Peeking",
                    "wave",
                    "peek",
                    "Hello",
                    "yo",
                    "greetings"
                ]
            }
        }
        "#;
        let line_webhook_event: LineWebhookEvent =
            serde_json::from_str(json).expect("Failed to deserialize");
        println!("line_webhook_event{:?}", line_webhook_event);
        LineWebhookRequest::new(destination.clone(), line_webhook_event);
        /*
         * sticker
         * メッセージスタンプの例
         */
        let json = r#"
        {
            "replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA",
            "type": "message",
            "mode": "active",
            "timestamp": 1462629479859,
            "source": {
                "type": "user",
                "userId": "U4af4980629..."
            },
            "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
            "deliveryContext": {
                "isRedelivery": false
            },
            "message": {
                "type": "sticker",
                "id": "123456789012345678",
                "quoteToken": "q3Plxr4AgKd...",
                "stickerId": "738839",
                "packageId": "12287",
                "stickerResourceType": "MESSAGE",
                "keywords": [
                    "Anticipation",
                    "Sparkle",
                    "Straight face",
                    "Staring",
                    "Thinking"
                ],
                "text": "今週末\n一緒に\n遊ぼうよ！"
            }
        }
        "#;
        let line_webhook_event: LineWebhookEvent =
            serde_json::from_str(json).expect("Failed to deserialize");
        println!("line_webhook_event{:?}", line_webhook_event);
        LineWebhookRequest::new(destination.clone(), line_webhook_event);
        /*
         * postback
         * 日時選択アクションのポストバックイベントの場合
         */
        let json = r#"
        {
            "replyToken": "b60d432864f44d079f6d8efe86cf404b",
            "type": "postback",
            "mode": "active",
            "source": {
                "userId": "U91eeaf62d...",
                "type": "user"
            },
            "timestamp": 1513669370317,
            "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
            "deliveryContext": {
                "isRedelivery": false
            },
            "postback": {
                "data": "storeId=12345",
                "params": {
                    "datetime": "2017-12-25T01:00"
                }
            }
        }
        "#;
        let line_webhook_event: LineWebhookEvent =
            serde_json::from_str(json).expect("Failed to deserialize");
        println!("line_webhook_event{:?}", line_webhook_event);
        LineWebhookRequest::new(destination, line_webhook_event);
    }
    /*
     * postback event
     */
    #[test]
    fn test_line_webhook_postback_event() {
        let destination = "line_id".to_string();
        /*
         * postback
         * 日時選択アクションのポストバックイベントの場合
         */
        let json = r#"
        {
            "replyToken": "b60d432864f44d079f6d8efe86cf404b",
            "type": "postback",
            "mode": "active",
            "source": {
                "userId": "U91eeaf62d...",
                "type": "user"
            },
            "timestamp": 1513669370317,
            "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
            "deliveryContext": {
                "isRedelivery": false
            },
            "postback": {
                "data": "storeId=12345",
                "params": {
                    "datetime": "2017-12-25T01:00"
                }
            }
        }
        "#;
        let line_webhook_event: LineWebhookEvent =
            serde_json::from_str(json).expect("Failed to deserialize");
        println!("line_webhook_event{:?}", line_webhook_event);
        LineWebhookRequest::new(destination.clone(), line_webhook_event);
        /*
         * postback
         * リッチメニュー切替アクションのポストバックイベントの場合
         */
        let json = r#"
        {
            "replyToken": "b60d432864f44d079f6d8efe86cf404b",
            "type": "postback",
            "mode": "active",
            "source": {
                "userId": "U91eeaf62d...",
                "type": "user"
            },
            "timestamp": 1619754620404,
            "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
            "deliveryContext": {
                "isRedelivery": false
            },
            "postback": {
                "data": "richmenu-changed-to-b",
                "params": {
                    "newRichMenuAliasId": "richmenu-alias-b",
                    "status": "SUCCESS"
                }
            }
        }
        "#;
        let line_webhook_event: LineWebhookEvent =
            serde_json::from_str(json).expect("Failed to deserialize");
        println!("line_webhook_event{:?}", line_webhook_event);
        LineWebhookRequest::new(destination.clone(), line_webhook_event);
        /*
         * postback
         * ボタンを押されたときのポストバックイベントの場合
         */
        let json = r#"
        {
            "replyToken": "b60d432864f44d079f6d8efe86cf404b",
            "type": "postback",
            "mode": "active",
            "source": {
                "userId": "U91eeaf62d...",
                "type": "user"
            },
            "timestamp": 1619754620404,
            "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
            "deliveryContext": {
                "isRedelivery": false
            },
            "postback": {
                "data": "richmenu-changed-to-b"
            }
        }
        "#;
        let line_webhook_event: LineWebhookEvent =
            serde_json::from_str(json).expect("Failed to deserialize");
        println!("line_webhook_event{:?}", line_webhook_event);
        LineWebhookRequest::new(destination, line_webhook_event);
    }
}
