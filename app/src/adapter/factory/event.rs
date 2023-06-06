use crate::{
    application::model::event::{
        CreateContentProvider, CreateEvent, CreateMessage, CreatePostbackParams,
        CreateStickerResourceType,
    },
    domain::{
        factory::event::EventFactory,
        model::{
            event::{
                Event, NewAudioMessage, NewContentProvider, NewDeliveryContext, NewEmoji, NewEvent,
                NewFileMessage, NewFollowEvent, NewImageMessage, NewImageSet, NewLocationMessage,
                NewMessage, NewMessageEvent, NewPostback, NewPostbackDatetimeParams,
                NewPostbackEvent, NewPostbackParams, NewPostbackRichMenuParams, NewStickerMessage,
                NewStickerResourceType, NewTextMessage, NewUnfollowEvent, NewVideoMessage,
                NewVideoPlayComplete, NewVideoPlayCompleteEvent,
            },
            primary_user_id::PrimaryUserId,
            Id,
        },
    },
};

use super::FactoryImpl;

impl EventFactory for FactoryImpl<Event> {
    fn create_new_event(
        &self,
        primary_user_id: PrimaryUserId,
        create_event: CreateEvent,
    ) -> NewEvent {
        let id = Id::<Event>::gen();
        let primary_user_id = primary_user_id.clone();
        let user_event = match create_event {
            CreateEvent::Follow(s) => NewEvent::Follow(NewFollowEvent {
                id,
                primary_user_id,
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
                primary_user_id,
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
                primary_user_id,
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
                    primary_user_id,
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
                    primary_user_id,
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
