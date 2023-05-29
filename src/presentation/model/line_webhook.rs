use crate::{application::model::{line_user_auth::CreateLineUserAuth, user_event::{CreateFollowEvent, DeliveryContext, EventType, CreateUnfollowEvent, CreateEvent, CreatePostbackEvent, Postback, PostbackParams,PostbackDatetimeParams, PostbackRichMenuParams, CreateVideoPlayCompleteEvent, VideoPlayComplete, CreateMessageEvent, Message, TextMessage, ImageMessage, ContentProvider, ImageSet, Emoji, VideoMessage, AudioMessage, FileMessage, LocationMessage, StickerMessage, CreateUserEvent}}, adapter::model::talk_room::MessageContent};
use serde::Deserialize;
use strum_macros::EnumString;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct LineWebhookRequest {
    destination: String,
    events: Vec<LineWebhookEvent>,
}

impl LineWebhookRequest {
    pub(in crate::presentation) fn get_events(&self) -> Vec<LineWebhookEvent> {
        self.events.clone()
    }
}

#[derive(Deserialize, Debug, Clone, Validate)]
pub(in crate::presentation) struct LineWebhookEvent {
    pub(in crate::presentation) r#type: LineWebhookEventType, // 限られた値に制限したい
    message: Option<LineWebhookMessage>,
    postback: Option<LineWebhookPostback>,
    video_play_complete: Option<LineWebhookVideoPlayComplete>,
    timestamp: i64,
    source: LineWebhookSource,
    #[serde(rename(deserialize = "replyToken"))]
    reply_token: String,
    mode: String,
    #[serde(rename(deserialize = "webhookEventId"))]
    webhook_event_id: String,
    #[serde(rename(deserialize = "deliveryContext"))]
    delivery_context: LineDeliveryContext,
}

// 文字列をEnumに変換する必要がある
#[derive(Deserialize, Debug, Clone, EnumString)]
pub(in crate::presentation) enum LineWebhookEventType {
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

#[derive(Deserialize, Debug, Clone)]
struct LineWebhookSource {
    r#type: String,
    user_id: String,
}

#[derive(Deserialize, Debug, Clone)]
struct LineDeliveryContext {
    #[serde(rename(deserialize = "isRedelivery"))]
    is_redelivery: bool,
}

#[derive(Deserialize, Debug, Clone)]
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

#[derive(Deserialize, Debug, Clone)]
struct LineWebhookTextMessage {
    id: String,
    text: String,
    emojis: Vec<LineWebhookEmoji>,
}

#[derive(Deserialize, Debug, Clone)]
struct LineWebhookEmoji {
    index: i32,
    length: i32,
    #[serde(rename(deserialize = "productId"))]
    product_id: String,
    #[serde(rename(deserialize = "emojiId"))]
    emoji_id: String,
}

#[derive(Deserialize, Debug, Clone)]
struct LineWebhookImageMessage {
    id: String,
    #[serde(rename(deserialize = "contentProvider"))]
    content_provider: LineWebhookContentProvider,
    image_set: LineWebhookImageSet,
}

#[derive(Deserialize, Debug, Clone)]
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

#[derive(Deserialize, Debug, Clone)]
struct LineWebhookImageSet {
    id: String,
    index: i32,
    length: i32,
}

#[derive(Deserialize, Debug, Clone)]
struct LineWebhookVideoMessage {
    id: String,
    duration: i32,
    #[serde(rename(deserialize = "contentProvider"))]
    content_provider: LineWebhookContentProvider,
}

#[derive(Deserialize, Debug, Clone)]
struct LineWebhookAudioMessage {
    id: String,
    duration: i32,
    #[serde(rename(deserialize = "contentProvider"))]
    content_provider: LineWebhookContentProvider,
}

#[derive(Deserialize, Debug, Clone)]
struct LineWebhookFileMessage {
    id: String,
    #[serde(rename(deserialize = "fileName"))]
    file_name: String,
    #[serde(rename(deserialize = "fileSize"))]
    file_size: i32,
}

#[derive(Deserialize, Debug, Clone)]
struct LineWebhookLocationMessage {
    id: String,
    title: String,
    address: String,
    latitude: f64,
    longitude: f64,
}

#[derive(Deserialize, Debug, Clone)]
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

#[derive(Deserialize, Debug, Clone, EnumString)]
enum LineWebhookStickerResourceType {
    #[strum(serialize = "STATIC")]
    Static,
    #[strum(serialize = "ANIMATION")]
    Animated,
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

#[derive(Deserialize, Debug, Clone)]
struct LineWebhookPostback {
    data: String,
    params: LineWebhookPostbackParams,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)] // JSONにタグ名を含まない
enum LineWebhookPostbackParams {
    Datetime(LineWebhookPostbackDatetimeParams),
    RichMenu(LineWebhookPostbackRichMenuParams),
}

#[derive(Deserialize, Debug, Clone)]
struct LineWebhookPostbackDatetimeParams {
    datetime: String,
}

#[derive(Deserialize, Debug, Clone)]
struct LineWebhookPostbackRichMenuParams {
    #[serde(rename(deserialize = "newRichMenuAliasId"))]
    new_rich_menu_alias_id: String,
    status: String,
}

#[derive(Deserialize, Debug, Clone)]
struct LineWebhookVideoPlayComplete {
    tracking_id: String,
}

impl From<LineWebhookEvent> for CreateUserEvent {
    fn from(s: LineWebhookEvent) -> Self {
        let create_line_user_auth = CreateLineUserAuth {
            user_id: s.source.user_id,
        };

        let create_event = match s.r#type {
            LineWebhookEventType::Follow => CreateEvent::Follow(
                CreateFollowEvent {
                    reply_token: s.reply_token,
                    delivery_context: DeliveryContext {
                        is_redelivery: s.delivery_context.is_redelivery,
                    },
                    event_type: EventType::Follow,
                    mode: s.mode,
                    timestamp: s.timestamp,
                }
            ),
            LineWebhookEventType::Unfollow => CreateEvent::Unfollow(
                CreateUnfollowEvent {
                    reply_token: s.reply_token,
                    delivery_context: DeliveryContext {
                        is_redelivery: s.delivery_context.is_redelivery,
                    },
                    event_type: EventType::Unfollow,
                    mode: s.mode,
                    timestamp: s.timestamp,
                }
            ),
            LineWebhookEventType::Postback => CreateEvent::Postback(
                CreatePostbackEvent {
                    reply_token: s.reply_token,
                    delivery_context: DeliveryContext {
                        is_redelivery: s.delivery_context.is_redelivery,
                    },
                    event_type: EventType::Postback,
                    postback: Postback {
                        data: s.postback.unwrap().data,
                        params: match s.postback.unwrap().params {
                            LineWebhookPostbackParams::Datetime(p) => PostbackParams::Datetime(
                                PostbackDatetimeParams {
                                    datetime: p.datetime,
                                }
                            ),
                            LineWebhookPostbackParams::RichMenu(p) => PostbackParams::RichMenu(
                                PostbackRichMenuParams {
                                    new_rich_menu_alias_id: p.new_rich_menu_alias_id,
                                    status: p.status,
                                }
                            ),
                        },
                    },
                    mode: s.mode,
                    timestamp: s.timestamp,
                }
            ),
            LineWebhookEventType::VideoPlayComplete => CreateEvent::VideoPlayComplete(
                {
                    CreateVideoPlayCompleteEvent {
                        reply_token: s.reply_token,
                        delivery_context: DeliveryContext {
                            is_redelivery: s.delivery_context.is_redelivery,
                        },
                        event_type: EventType::VideoPlayComplete,
                        video_play_complete: VideoPlayComplete {
                            tracking_id: s.video_play_complete.unwrap().tracking_id,
                        },
                        mode: s.mode,
                        timestamp: s.timestamp,
                    }
                }
            ),
            LineWebhookEventType::Message => CreateEvent::Message(
                {
                    CreateMessageEvent {
                        reply_token: s.reply_token,
                        delivery_context: DeliveryContext {
                            is_redelivery: s.delivery_context.is_redelivery,
                        },
                        event_type: EventType::Message,
                        message: match s.message.unwrap() {
                            LineWebhookMessage::Text(m) => Message::Text(
                                TextMessage {
                                    id: m.id,
                                    text: m.text,
                                    emojis: m.emojis.iter().map(|e| Emoji {
                                        index: e.index,
                                        length: e.length,
                                        product_id: e.product_id,
                                        emoji_id: e.emoji_id,
                                    }).collect(),
                                }
                            ),
                            LineWebhookMessage::Image(m) => Message::Image(
                                ImageMessage {
                                    id: m.id,
                                    content_provider: match m.content_provider {
                                        LineWebhookContentProvider::Line => ContentProvider::Line,
                                        LineWebhookContentProvider::External { original_content_url, preview_image_url } => ContentProvider::External {
                                            original_content_url: original_content_url,
                                            preview_image_url: preview_image_url,
                                        },
                                    },
                                    image_set: ImageSet {
                                        id: m.image_set.id,
                                        index: m.image_set.index,
                                        length: m.image_set.length,
                                    },
                                },
                            ),
                            LineWebhookMessage::Video(m) => Message::Video(
                                VideoMessage {
                                    id: m.id,
                                    duration: m.duration,
                                    content_provider: match m.content_provider {
                                        LineWebhookContentProvider::Line => ContentProvider::Line,
                                        LineWebhookContentProvider::External { original_content_url, preview_image_url } => ContentProvider::External {
                                            original_content_url: original_content_url,
                                            preview_image_url: preview_image_url,
                                        },
                                    },
                                },
                            ),
                            LineWebhookMessage::Audio(m) => Message::Audio(
                                AudioMessage {
                                    id: m.id,
                                    duration: m.duration,
                                    content_provider: match m.content_provider {
                                        LineWebhookContentProvider::Line => ContentProvider::Line,
                                        LineWebhookContentProvider::External { original_content_url, preview_image_url } => ContentProvider::External {
                                            original_content_url: original_content_url,
                                            preview_image_url: preview_image_url,
                                        },
                                    },
                                },
                            ),
                            LineWebhookMessage::File(m) => Message::File(
                                FileMessage {
                                    id: m.id,
                                    file_name: m.file_name,
                                    file_size: m.file_size,
                                },
                            ),
                            LineWebhookMessage::Location(m) => Message::Location(
                                LocationMessage {
                                    id: m.id,
                                    title: m.title,
                                    address: m.address,
                                    latitude: m.latitude,
                                    longitude: m.longitude,
                                },
                            ),
                            LineWebhookMessage::Sticker(m) => Message::Sticker(
                                StickerMessage {
                                    id: m.id,
                                    package_id: m.package_id,
                                    sticker_id: m.sticker_id,
                                    sticker_resource_type: match m.sticker_resource_type {
                                        LineWebhookStickerResourceType::Static => crate::application::model::user_event::StickerResourceType::Static,
                                        LineWebhookStickerResourceType::Animated => crate::application::model::user_event::StickerResourceType::Animated,
                                        LineWebhookStickerResourceType::Sound => crate::application::model::user_event::StickerResourceType::Sound,
                                        LineWebhookStickerResourceType::AnimationSound => crate::application::model::user_event::StickerResourceType::AnimationSound,
                                        LineWebhookStickerResourceType::Popup => crate::application::model::user_event::StickerResourceType::Popup,
                                        LineWebhookStickerResourceType::PupupSound => crate::application::model::user_event::StickerResourceType::PupupSound,
                                        LineWebhookStickerResourceType::Custom => crate::application::model::user_event::StickerResourceType::Custom,
                                        LineWebhookStickerResourceType::Message => crate::application::model::user_event::StickerResourceType::Message,
                                    },
                                    keywords: m.keywords,
                                    text: m.text,
                                },
                            )

                        },
                        mode: s.mode,
                        timestamp: s.timestamp,
                    }
                }
            ),
        };

        CreateUserEvent {
            create_line_user_auth: CreateLineUserAuth {
                user_id: s.source.user_id,
            },
            create_user_event: create_event
        }
    }
}
