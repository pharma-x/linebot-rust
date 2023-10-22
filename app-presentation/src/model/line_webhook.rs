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

#[derive(new, Serialize, Deserialize, Debug, Validate, Clone)]
pub struct LineWebhookRequests {
    pub destination: String,
    pub events: Vec<LineWebhookEvent>,
}

#[derive(Debug, Validate, Clone)]
pub struct LineWebhookRequest {
    pub destination: String,
    pub event: LineWebhookEvent,
}

#[derive(Serialize, Deserialize, Debug, Clone, Display)]
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
pub struct LineWebhookFollowEvent {
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
    postback: Option<LineWebhookPostback>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LineWebhookPostback {
    data: String,
    params: LineWebhookPostbackParams,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)] // JSONにタグ名を含まない
enum LineWebhookPostbackParams {
    Datetime(LineWebhookPostbackDatetimeParams),
    RichMenu(LineWebhookPostbackRichMenuParams),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LineWebhookPostbackDatetimeParams {
    datetime: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LineWebhookPostbackRichMenuParams {
    #[serde(rename(deserialize = "newRichMenuAliasId"))]
    new_rich_menu_alias_id: String,
    status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
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
struct LineWebhookVideoPlayComplete {
    tracking_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
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
    message: Option<LineWebhookMessage>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Display)]
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
pub struct LineWebhookUserSource {
    #[serde(rename(deserialize = "userId"))]
    user_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LineWebhookGroupSource {
    #[serde(rename(deserialize = "groupId"))]
    group_id: String,
    #[serde(rename(deserialize = "userId"))]
    user_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LineWebhookRoomSource {
    #[serde(rename(deserialize = "roomId"))]
    room_id: String,
    #[serde(rename(deserialize = "userId"))]
    user_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LineDeliveryContext {
    #[serde(rename(deserialize = "isRedelivery"))]
    is_redelivery: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
struct LineWebhookTextMessage {
    id: String,
    text: String,
    emojis: Vec<LineWebhookEmoji>,
    mention: Option<LineWebhookMention>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LineWebhookEmoji {
    index: i32,
    length: i32,
    #[serde(rename(deserialize = "productId"))]
    product_id: String,
    #[serde(rename(deserialize = "emojiId"))]
    emoji_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LineWebhookMention {
    mentionees: Vec<LineWebhookMentionee>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
enum LineWebhookMentionee {
    LineWebhookUserMentionee,
    LineWebhookAllMentionee,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LineWebhookUserMentionee {
    index: i32,
    length: i32,
    #[serde(rename(deserialize = "userId"))]
    user_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LineWebhookAllMentionee {
    index: i32,
    length: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LineWebhookImageMessage {
    id: String,
    #[serde(rename(deserialize = "contentProvider"))]
    content_provider: LineWebhookContentProvider,
    image_set: Option<LineWebhookImageSet>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
struct LineWebhookImageSet {
    id: String,
    index: i32,
    length: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LineWebhookVideoMessage {
    id: String,
    duration: i32,
    #[serde(rename(deserialize = "contentProvider"))]
    content_provider: LineWebhookContentProvider,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LineWebhookAudioMessage {
    id: String,
    duration: i32,
    #[serde(rename(deserialize = "contentProvider"))]
    content_provider: LineWebhookContentProvider,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LineWebhookFileMessage {
    id: String,
    #[serde(rename(deserialize = "fileName"))]
    file_name: String,
    #[serde(rename(deserialize = "fileSize"))]
    file_size: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LineWebhookLocationMessage {
    id: String,
    title: String,
    address: String,
    latitude: f64,
    longitude: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
            LineWebhookEvent::Follow(s) => CreateEvent::Follow(CreateFollowEvent {
                reply_token: s.reply_token,
                delivery_context: CreateDeliveryContext {
                    is_redelivery: s.delivery_context.is_redelivery,
                },
                mode: s.mode,
                webhook_event_id: s.webhook_event_id,
                timestamp: s.timestamp,
            }),
            LineWebhookEvent::Unfollow(s) => CreateEvent::Unfollow(CreateUnfollowEvent {
                reply_token: s.reply_token,
                delivery_context: CreateDeliveryContext {
                    is_redelivery: s.delivery_context.is_redelivery,
                },
                mode: s.mode,
                webhook_event_id: s.webhook_event_id,
                timestamp: s.timestamp,
            }),
            LineWebhookEvent::Postback(s) => CreateEvent::Postback(CreatePostbackEvent {
                reply_token: s.reply_token,
                delivery_context: CreateDeliveryContext {
                    is_redelivery: s.delivery_context.is_redelivery,
                },
                postback: CreatePostback {
                    data: s.postback.clone().unwrap().data,
                    params: match s.postback.clone().unwrap().params {
                        LineWebhookPostbackParams::Datetime(p) => {
                            CreatePostbackParams::Datetime(CreatePostbackDatetimeParams {
                                datetime: p.datetime,
                            })
                        }
                        LineWebhookPostbackParams::RichMenu(p) => {
                            CreatePostbackParams::RichMenu(CreatePostbackRichMenuParams {
                                new_rich_menu_alias_id: p.new_rich_menu_alias_id,
                                status: p.status,
                            })
                        }
                    },
                },
                mode: s.mode,
                webhook_event_id: s.webhook_event_id,
                timestamp: s.timestamp,
            }),
            LineWebhookEvent::VideoPlayComplete(s) => CreateEvent::VideoPlayComplete({
                CreateVideoPlayCompleteEvent {
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
            }),
            LineWebhookEvent::Message(s) => CreateEvent::Message({
                CreateMessageEvent {
                    reply_token: s.reply_token,
                    delivery_context: CreateDeliveryContext {
                        is_redelivery: s.delivery_context.is_redelivery,
                    },
                    message: match s.message.unwrap() {
                        LineWebhookMessage::Text(m) => CreateMessage::Text(CreateTextMessage {
                            id: m.id,
                            text: m.text,
                            emojis: m
                                .emojis
                                .iter()
                                .map(|e| CreateEmoji {
                                    index: e.index,
                                    length: e.length,
                                    product_id: e.product_id.clone(),
                                    emoji_id: e.emoji_id.clone(),
                                })
                                .collect(),
                        }),
                        LineWebhookMessage::Image(m) => CreateMessage::Image(CreateImageMessage {
                            id: m.id,
                            content_provider: match m.content_provider {
                                LineWebhookContentProvider::Line => CreateContentProvider::Line,
                                LineWebhookContentProvider::External {
                                    original_content_url,
                                    preview_image_url,
                                } => CreateContentProvider::External {
                                    original_content_url,
                                    preview_image_url,
                                },
                            },
                            image_set: m.image_set.map(|i| CreateImageSet {
                                id: i.id,
                                index: i.index,
                                length: i.length,
                            }),
                        }),
                        LineWebhookMessage::Video(m) => CreateMessage::Video(CreateVideoMessage {
                            id: m.id,
                            duration: m.duration,
                            content_provider: match m.content_provider {
                                LineWebhookContentProvider::Line => CreateContentProvider::Line,
                                LineWebhookContentProvider::External {
                                    original_content_url,
                                    preview_image_url,
                                } => CreateContentProvider::External {
                                    original_content_url,
                                    preview_image_url,
                                },
                            },
                        }),
                        LineWebhookMessage::Audio(m) => CreateMessage::Audio(CreateAudioMessage {
                            id: m.id,
                            duration: m.duration,
                            content_provider: match m.content_provider {
                                LineWebhookContentProvider::Line => CreateContentProvider::Line,
                                LineWebhookContentProvider::External {
                                    original_content_url,
                                    preview_image_url,
                                } => CreateContentProvider::External {
                                    original_content_url,
                                    preview_image_url,
                                },
                            },
                        }),
                        LineWebhookMessage::File(m) => CreateMessage::File(CreateFileMessage {
                            id: m.id,
                            file_name: m.file_name,
                            file_size: m.file_size,
                        }),
                        LineWebhookMessage::Location(m) => {
                            CreateMessage::Location(CreateLocationMessage {
                                id: m.id,
                                title: m.title,
                                address: m.address,
                                latitude: m.latitude,
                                longitude: m.longitude,
                            })
                        }
                        LineWebhookMessage::Sticker(m) => {
                            CreateMessage::Sticker(CreateStickerMessage {
                                id: m.id,
                                package_id: m.package_id,
                                sticker_id: m.sticker_id,
                                sticker_resource_type: match m.sticker_resource_type {
                                    LineWebhookStickerResourceType::Static => {
                                        CreateStickerResourceType::Static
                                    }
                                    LineWebhookStickerResourceType::Animation => {
                                        CreateStickerResourceType::Animation
                                    }
                                    LineWebhookStickerResourceType::Sound => {
                                        CreateStickerResourceType::Sound
                                    }
                                    LineWebhookStickerResourceType::AnimationSound => {
                                        CreateStickerResourceType::AnimationSound
                                    }
                                    LineWebhookStickerResourceType::Popup => {
                                        CreateStickerResourceType::Popup
                                    }
                                    LineWebhookStickerResourceType::PupupSound => {
                                        CreateStickerResourceType::PupupSound
                                    }
                                    LineWebhookStickerResourceType::Custom => {
                                        CreateStickerResourceType::Custom
                                    }
                                    LineWebhookStickerResourceType::Message => {
                                        CreateStickerResourceType::Message
                                    }
                                },
                                keywords: m.keywords,
                                text: m.text,
                            })
                        }
                    },
                    mode: s.mode,
                    webhook_event_id: s.webhook_event_id,
                    timestamp: s.timestamp,
                }
            }),
        };

        CreateUserEvent {
            create_line_user_auth: CreateLineUserAuth {
                user_id: r.destination,
            },
            create_event,
        }
    }
}
