use derive_new::new;

use crate::{
    application::model::event::{
        CreateContentProvider, CreateEvent, CreateMessage, CreatePostbackParams,
        CreateStickerResourceType,
    },
    domain::model::{
        event::{
            AudioMessage, ContentProvider, DeliveryContext, Emoji, Event, FileMessage, FollowEvent,
            ImageMessage, ImageSet, LocationMessage, Message, MessageEvent, Postback,
            PostbackDatetimeParams, PostbackEvent, PostbackParams, PostbackRichMenuParams,
            StickerMessage, StickerResourceType, TextMessage, UnfollowEvent, VideoMessage,
            VideoPlayComplete, VideoPlayCompleteEvent,
        },
        talk_room::TalkRoom,
        Id,
    },
};

#[derive(new)]
pub struct EventFactory {}

impl EventFactory {
    pub fn create_event(&self, talk_room: TalkRoom, s: CreateEvent) -> Event {
        let talk_room_id = talk_room.id;
        let id = Id::<Event>::gen().value.to_string();

        let user_event = match s {
            CreateEvent::Follow(s) => Event::Follow(FollowEvent {
                id,
                talk_room_id,
                reply_token: s.reply_token,
                delivery_context: DeliveryContext {
                    is_redelivery: s.delivery_context.is_redelivery,
                },
                mode: s.mode,
                webhook_event_id: s.webhook_event_id,
                timestamp: s.timestamp,
            }),
            CreateEvent::Unfollow(s) => Event::Unfollow(UnfollowEvent {
                id,
                talk_room_id,
                reply_token: s.reply_token,
                delivery_context: DeliveryContext {
                    is_redelivery: s.delivery_context.is_redelivery,
                },
                mode: s.mode,
                webhook_event_id: s.webhook_event_id,
                timestamp: s.timestamp,
            }),
            CreateEvent::Postback(s) => Event::Postback(PostbackEvent {
                id,
                talk_room_id,
                reply_token: s.reply_token,
                delivery_context: DeliveryContext {
                    is_redelivery: s.delivery_context.is_redelivery,
                },
                postback: Postback {
                    data: s.postback.data,
                    params: match s.postback.params {
                        CreatePostbackParams::Datetime(p) => {
                            PostbackParams::Datetime(PostbackDatetimeParams {
                                datetime: p.datetime,
                            })
                        }
                        CreatePostbackParams::RichMenu(p) => {
                            PostbackParams::RichMenu(PostbackRichMenuParams {
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
            CreateEvent::VideoPlayComplete(s) => Event::VideoPlayComplete({
                VideoPlayCompleteEvent {
                    id,
                    talk_room_id,
                    reply_token: s.reply_token,
                    delivery_context: DeliveryContext {
                        is_redelivery: s.delivery_context.is_redelivery,
                    },
                    video_play_complete: VideoPlayComplete {
                        tracking_id: s.video_play_complete.tracking_id,
                    },
                    mode: s.mode,
                    webhook_event_id: s.webhook_event_id,
                    timestamp: s.timestamp,
                }
            }),
            CreateEvent::Message(s) => Event::Message({
                MessageEvent {
                    id,
                    talk_room_id,
                    reply_token: s.reply_token,
                    delivery_context: DeliveryContext {
                        is_redelivery: s.delivery_context.is_redelivery,
                    },
                    message: match s.message {
                        CreateMessage::Text(m) => Message::Text(TextMessage {
                            id: m.id,
                            text: m.text,
                            emojis: m
                                .emojis
                                .iter()
                                .map(|e| Emoji {
                                    index: e.index,
                                    length: e.length,
                                    product_id: e.product_id.clone(),
                                    emoji_id: e.emoji_id.clone(),
                                })
                                .collect(),
                        }),
                        CreateMessage::Image(m) => Message::Image(ImageMessage {
                            id: m.id,
                            content_provider: match m.content_provider {
                                CreateContentProvider::Line => ContentProvider::Line,
                                CreateContentProvider::External {
                                    original_content_url,
                                    preview_image_url,
                                } => ContentProvider::External {
                                    original_content_url,
                                    preview_image_url,
                                },
                            },
                            image_set: ImageSet {
                                id: m.image_set.id,
                                index: m.image_set.index,
                                length: m.image_set.length,
                            },
                        }),
                        CreateMessage::Video(m) => Message::Video(VideoMessage {
                            id: m.id,
                            duration: m.duration,
                            content_provider: match m.content_provider {
                                CreateContentProvider::Line => ContentProvider::Line,
                                CreateContentProvider::External {
                                    original_content_url,
                                    preview_image_url,
                                } => ContentProvider::External {
                                    original_content_url,
                                    preview_image_url,
                                },
                            },
                        }),
                        CreateMessage::Audio(m) => Message::Audio(AudioMessage {
                            id: m.id,
                            duration: m.duration,
                            content_provider: match m.content_provider {
                                CreateContentProvider::Line => ContentProvider::Line,
                                CreateContentProvider::External {
                                    original_content_url,
                                    preview_image_url,
                                } => ContentProvider::External {
                                    original_content_url,
                                    preview_image_url,
                                },
                            },
                        }),
                        CreateMessage::File(m) => Message::File(FileMessage {
                            id: m.id,
                            file_name: m.file_name,
                            file_size: m.file_size,
                        }),
                        CreateMessage::Location(m) => Message::Location(LocationMessage {
                            id: m.id,
                            title: m.title,
                            address: m.address,
                            latitude: m.latitude,
                            longitude: m.longitude,
                        }),
                        CreateMessage::Sticker(m) => Message::Sticker(StickerMessage {
                            id: m.id,
                            package_id: m.package_id,
                            sticker_id: m.sticker_id,
                            sticker_resource_type: match m.sticker_resource_type {
                                CreateStickerResourceType::Static => StickerResourceType::Static,
                                CreateStickerResourceType::Animation => {
                                    StickerResourceType::Animation
                                }
                                CreateStickerResourceType::Sound => StickerResourceType::Sound,
                                CreateStickerResourceType::AnimationSound => {
                                    StickerResourceType::AnimationSound
                                }
                                CreateStickerResourceType::Popup => StickerResourceType::Popup,
                                CreateStickerResourceType::PupupSound => {
                                    StickerResourceType::PupupSound
                                }
                                CreateStickerResourceType::Custom => StickerResourceType::Custom,
                                CreateStickerResourceType::Message => StickerResourceType::Message,
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
