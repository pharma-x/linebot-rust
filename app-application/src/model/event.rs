use crate::model::line_user_auth::CreateLineUserAuth;
use derive_new::new;

use domain::model::event::NewEvent;

#[derive(new, Clone)]
pub struct CreateUserEvent {
    pub create_line_user_auth: CreateLineUserAuth,
    pub create_event: CreateEvent,
}

#[derive(new, Clone)]
pub enum CreateEvent {
    Follow(CreateFollowEvent),
    Unfollow(CreateUnfollowEvent),
    Postback(CreatePostbackEvent),
    VideoPlayComplete(CreateVideoPlayCompleteEvent),
    Message(CreateMessageEvent),
}

#[derive(new, Clone)]
pub struct CreateFollowEvent {
    pub reply_token: String,
    pub delivery_context: CreateDeliveryContext,
    pub mode: String,
    pub webhook_event_id: String,
    pub timestamp: i64,
}

#[derive(new, Clone)]
pub struct CreateUnfollowEvent {
    pub reply_token: String,
    pub delivery_context: CreateDeliveryContext,
    pub mode: String,
    pub webhook_event_id: String,
    pub timestamp: i64,
}

#[derive(new, Clone)]
pub struct CreatePostbackEvent {
    pub reply_token: String,
    pub delivery_context: CreateDeliveryContext,
    pub postback: CreatePostback,
    pub mode: String,
    pub webhook_event_id: String,
    pub timestamp: i64,
}

#[derive(new, Clone)]
pub struct CreateVideoPlayCompleteEvent {
    pub reply_token: String,
    pub delivery_context: CreateDeliveryContext,
    pub video_play_complete: CreateVideoPlayComplete,
    pub mode: String,
    pub webhook_event_id: String,
    pub timestamp: i64,
}

#[derive(new, Clone)]
pub struct CreateMessageEvent {
    pub reply_token: String,
    pub delivery_context: CreateDeliveryContext,
    pub message: CreateMessage,
    pub mode: String,
    pub webhook_event_id: String,
    pub timestamp: i64,
}

#[derive(new, Clone)]
pub enum CreateEventType {
    Message,
    Follow,
    Unfollow,
    Postback,
    VideoPlayComplete,
}

#[derive(new, Debug, Clone)]
pub struct CreateDeliveryContext {
    pub is_redelivery: bool,
}

#[derive(new, Clone)]
pub struct CreatePostback {
    pub data: String,
    pub params: CreatePostbackParams,
}

#[derive(new, Clone)]
pub enum CreatePostbackParams {
    Datetime(CreatePostbackDatetimeParams),
    RichMenu(CreatePostbackRichMenuParams),
}

#[derive(new, Clone)]
pub struct CreatePostbackDatetimeParams {
    pub datetime: String,
}

#[derive(new, Clone)]
pub struct CreatePostbackRichMenuParams {
    pub new_rich_menu_alias_id: String,
    pub status: String,
}

#[derive(new, Clone)]
pub struct CreateVideoPlayComplete {
    pub tracking_id: String,
}

#[derive(new, Clone)]
pub enum CreateMessage {
    Text(CreateTextMessage),
    Image(CreateImageMessage),
    Video(CreateVideoMessage),
    Audio(CreateAudioMessage),
    File(CreateFileMessage),
    Location(CreateLocationMessage),
    Sticker(CreateStickerMessage),
}

#[derive(new, Clone)]
pub struct CreateTextMessage {
    pub id: String,
    pub text: String,
    pub emojis: Vec<CreateEmoji>,
}

#[derive(new, Clone)]
pub struct CreateEmoji {
    pub index: i32,
    pub length: i32,
    pub product_id: String,
    pub emoji_id: String,
}

#[derive(new, Clone)]
pub struct CreateImageMessage {
    pub id: String,
    pub content_provider: CreateContentProvider,
    pub image_set: CreateImageSet,
}

#[derive(new, Clone)]
pub enum CreateContentProvider {
    Line,
    External {
        original_content_url: String,
        preview_image_url: Option<String>,
    },
}

#[derive(new, Clone)]
pub struct CreateImageSet {
    pub id: String,
    pub index: i32,
    pub length: i32,
}

#[derive(new, Clone)]
pub struct CreateVideoMessage {
    pub id: String,
    pub duration: i32,
    pub content_provider: CreateContentProvider,
}

#[derive(new, Clone)]
pub struct CreateAudioMessage {
    pub id: String,
    pub duration: i32,
    pub content_provider: CreateContentProvider,
}

#[derive(new, Clone)]
pub struct CreateFileMessage {
    pub id: String,
    pub file_name: String,
    pub file_size: i32,
}

#[derive(new, Clone)]
pub struct CreateLocationMessage {
    pub id: String,
    pub title: String,
    pub address: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(new, Clone)]
pub struct CreateStickerMessage {
    pub id: String,
    pub package_id: String,
    pub sticker_id: String,
    pub sticker_resource_type: CreateStickerResourceType,
    pub keywords: Option<Vec<String>>,
    pub text: Option<String>,
}

#[derive(new, Clone)]
pub enum CreateStickerResourceType {
    Static,
    Animation,
    Sound,
    AnimationSound,
    Popup,
    PupupSound,
    Custom,
    Message,
}

From<(TalkRoom, CreateEvent)> for NewEvent {
    fn into(s: (TalkRoom, CreateEvent)) -> Self {
        let id = Id::<Event>::gen();
        let talk_room_id = s.0.id;
        let user_event = match create_event {
            CreateEvent::Follow(s) => NewEvent::Follow(NewFollowEvent {
                id,
                talk_room_id,
                reply_token: s.reply_token,
                delivery_context: NewDeliveryContext {
                    is_redelivery: s.delivery_context.is_redelivery,
                },
                mode: s.mode,
                webhook_event_id: s.webhook_event_id,
                timestamp: s.timestamp,
            }),
            CreateEvent::Unfollow(s) => NewEvent::Unfollow(NewUnfollowEvent {
                id,
                talk_room_id,
                reply_token: s.reply_token,
                delivery_context: NewDeliveryContext {
                    is_redelivery: s.delivery_context.is_redelivery,
                },
                mode: s.mode,
                webhook_event_id: s.webhook_event_id,
                timestamp: s.timestamp,
            }),
            CreateEvent::Postback(s) => NewEvent::Postback(NewPostbackEvent {
                id,
                talk_room_id,
                reply_token: s.reply_token,
                delivery_context: NewDeliveryContext {
                    is_redelivery: s.delivery_context.is_redelivery,
                },
                postback: NewPostback {
                    data: s.postback.data,
                    params: match s.postback.params {
                        CreatePostbackParams::Datetime(p) => {
                            NewPostbackParams::Datetime(NewPostbackDatetimeParams {
                                datetime: p.datetime,
                            })
                        }
                        CreatePostbackParams::RichMenu(p) => {
                            NewPostbackParams::RichMenu(NewPostbackRichMenuParams {
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
            CreateEvent::VideoPlayComplete(s) => NewEvent::VideoPlayComplete({
                NewVideoPlayCompleteEvent {
                    id,
                    talk_room_id,
                    reply_token: s.reply_token,
                    delivery_context: NewDeliveryContext {
                        is_redelivery: s.delivery_context.is_redelivery,
                    },
                    video_play_complete: NewVideoPlayComplete {
                        tracking_id: s.video_play_complete.tracking_id,
                    },
                    mode: s.mode,
                    webhook_event_id: s.webhook_event_id,
                    timestamp: s.timestamp,
                }
            }),
            CreateEvent::Message(s) => NewEvent::Message({
                NewMessageEvent {
                    id,
                    talk_room_id,
                    reply_token: s.reply_token,
                    delivery_context: NewDeliveryContext {
                        is_redelivery: s.delivery_context.is_redelivery,
                    },
                    message: match s.message {
                        CreateMessage::Text(m) => NewMessage::Text(NewTextMessage {
                            id: m.id,
                            text: m.text,
                            emojis: m
                                .emojis
                                .iter()
                                .map(|e| NewEmoji {
                                    index: e.index,
                                    length: e.length,
                                    product_id: e.product_id.clone(),
                                    emoji_id: e.emoji_id.clone(),
                                })
                                .collect(),
                        }),
                        CreateMessage::Image(m) => NewMessage::Image(NewImageMessage {
                            id: m.id,
                            content_provider: match m.content_provider {
                                CreateContentProvider::Line => NewContentProvider::Line,
                                CreateContentProvider::External {
                                    original_content_url,
                                    preview_image_url,
                                } => NewContentProvider::External {
                                    original_content_url,
                                    preview_image_url,
                                },
                            },
                            image_set: NewImageSet {
                                id: m.image_set.id,
                                index: m.image_set.index,
                                length: m.image_set.length,
                            },
                        }),
                        CreateMessage::Video(m) => NewMessage::Video(NewVideoMessage {
                            id: m.id,
                            duration: m.duration,
                            content_provider: match m.content_provider {
                                CreateContentProvider::Line => NewContentProvider::Line,
                                CreateContentProvider::External {
                                    original_content_url,
                                    preview_image_url,
                                } => NewContentProvider::External {
                                    original_content_url,
                                    preview_image_url,
                                },
                            },
                        }),
                        CreateMessage::Audio(m) => NewMessage::Audio(NewAudioMessage {
                            id: m.id,
                            duration: m.duration,
                            content_provider: match m.content_provider {
                                CreateContentProvider::Line => NewContentProvider::Line,
                                CreateContentProvider::External {
                                    original_content_url,
                                    preview_image_url,
                                } => NewContentProvider::External {
                                    original_content_url,
                                    preview_image_url,
                                },
                            },
                        }),
                        CreateMessage::File(m) => NewMessage::File(NewFileMessage {
                            id: m.id,
                            file_name: m.file_name,
                            file_size: m.file_size,
                        }),
                        CreateMessage::Location(m) => NewMessage::Location(NewLocationMessage {
                            id: m.id,
                            title: m.title,
                            address: m.address,
                            latitude: m.latitude,
                            longitude: m.longitude,
                        }),
                        CreateMessage::Sticker(m) => NewMessage::Sticker(NewStickerMessage {
                            id: m.id,
                            package_id: m.package_id,
                            sticker_id: m.sticker_id,
                            sticker_resource_type: match m.sticker_resource_type {
                                CreateStickerResourceType::Static => NewStickerResourceType::Static,
                                CreateStickerResourceType::Animation => {
                                    NewStickerResourceType::Animation
                                }
                                CreateStickerResourceType::Sound => NewStickerResourceType::Sound,
                                CreateStickerResourceType::AnimationSound => {
                                    NewStickerResourceType::AnimationSound
                                }
                                CreateStickerResourceType::Popup => NewStickerResourceType::Popup,
                                CreateStickerResourceType::PupupSound => {
                                    NewStickerResourceType::PupupSound
                                }
                                CreateStickerResourceType::Custom => NewStickerResourceType::Custom,
                                CreateStickerResourceType::Message => {
                                    NewStickerResourceType::Message
                                }
                            },
                            keywords: m.keywords,
                            text: m.text,
                        }),
                    },
                    mode: s.mode,
                    webhook_event_id: s.webhook_event_id,
                    timestamp: s.timestamp,
                }
            }),
        };
        user_event
    }
}
