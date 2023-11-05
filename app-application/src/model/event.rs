use crate::model::line_user_auth::CreateLineUserAuth;
use chrono::{Local, TimeZone};
use derive_new::new;

use domain::model::{
    event::{
        NewAudioMessage, NewContentProvider, NewDeliveryContext, NewEmoji, NewEvent,
        NewExternalContentProvider, NewFileMessage, NewFollowEvent, NewImageMessage, NewImageSet,
        NewLocationMessage, NewMessage, NewMessageEvent, NewPostback, NewPostbackDatetimeParams,
        NewPostbackEvent, NewPostbackParams, NewPostbackRichMenuParams, NewStickerMessage,
        NewStickerResourceType, NewTextMessage, NewUnfollowEvent, NewVideoMessage,
        NewVideoPlayComplete, NewVideoPlayCompleteEvent,
    },
    Id,
};
use rust_decimal::{prelude::FromPrimitive, Decimal};

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
pub enum CreatePostbackDatetimeParams {
    DateTime(String),
    Date(String),
    Time(String),
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
    pub image_set: Option<CreateImageSet>,
}

#[derive(new, Clone)]
pub enum CreateContentProvider {
    Line,
    External(CreateExternalContentProvider),
}

#[derive(new, Clone)]
pub struct CreateExternalContentProvider {
    pub original_content_url: String,
    pub preview_image_url: Option<String>,
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
    PopupSound,
    Custom,
    Message,
}

impl From<CreateEvent> for NewEvent {
    fn from(s: CreateEvent) -> Self {
        match s {
            CreateEvent::Follow(s) => NewEvent::Follow(s.into()),
            CreateEvent::Unfollow(s) => NewEvent::Unfollow(s.into()),
            CreateEvent::Postback(s) => NewEvent::Postback(s.into()),
            CreateEvent::VideoPlayComplete(s) => NewEvent::VideoPlayComplete(s.into()),
            CreateEvent::Message(s) => NewEvent::Message(s.into()),
        }
    }
}

impl From<CreateDeliveryContext> for NewDeliveryContext {
    fn from(s: CreateDeliveryContext) -> Self {
        Self {
            is_redelivery: s.is_redelivery,
        }
    }
}

impl From<CreateFollowEvent> for NewFollowEvent {
    fn from(s: CreateFollowEvent) -> Self {
        let id = Id::gen();
        let created_at = Local.timestamp_opt(s.timestamp / 1000, 0).unwrap();
        println!("created_at: {:?}", created_at);
        Self {
            id,
            reply_token: s.reply_token,
            delivery_context: NewDeliveryContext::from(s.delivery_context),
            mode: s.mode,
            webhook_event_id: s.webhook_event_id,
            created_at,
        }
    }
}

impl From<CreateUnfollowEvent> for NewUnfollowEvent {
    fn from(s: CreateUnfollowEvent) -> Self {
        let id = Id::gen();
        let created_at = Local.timestamp_opt(s.timestamp / 1000, 0).unwrap();
        Self {
            id,
            delivery_context: NewDeliveryContext::from(s.delivery_context),
            mode: s.mode,
            webhook_event_id: s.webhook_event_id,
            created_at,
        }
    }
}

impl From<CreatePostbackEvent> for NewPostbackEvent {
    fn from(s: CreatePostbackEvent) -> Self {
        let id = Id::gen();
        let created_at = Local.timestamp_opt(s.timestamp / 1000, 0).unwrap();
        Self {
            id,
            reply_token: s.reply_token,
            delivery_context: NewDeliveryContext::from(s.delivery_context),
            postback: NewPostback::from(s.postback),
            mode: s.mode,
            webhook_event_id: s.webhook_event_id,
            created_at,
        }
    }
}

impl From<CreatePostback> for NewPostback {
    fn from(s: CreatePostback) -> Self {
        Self {
            data: s.data,
            params: NewPostbackParams::from(s.params),
        }
    }
}

impl From<CreatePostbackParams> for NewPostbackParams {
    fn from(s: CreatePostbackParams) -> Self {
        match s {
            CreatePostbackParams::Datetime(s) => {
                NewPostbackParams::Datetime(NewPostbackDatetimeParams::from(s))
            }
            CreatePostbackParams::RichMenu(s) => {
                NewPostbackParams::RichMenu(NewPostbackRichMenuParams::from(s))
            }
        }
    }
}

impl From<CreatePostbackDatetimeParams> for NewPostbackDatetimeParams {
    fn from(s: CreatePostbackDatetimeParams) -> Self {
        match s {
            CreatePostbackDatetimeParams::DateTime(s) => NewPostbackDatetimeParams::DateTime(s),
            CreatePostbackDatetimeParams::Date(s) => NewPostbackDatetimeParams::Date(s),
            CreatePostbackDatetimeParams::Time(s) => NewPostbackDatetimeParams::Time(s),
        }
    }
}

impl From<CreatePostbackRichMenuParams> for NewPostbackRichMenuParams {
    fn from(s: CreatePostbackRichMenuParams) -> Self {
        Self {
            new_rich_menu_alias_id: s.new_rich_menu_alias_id,
            status: s.status,
        }
    }
}

impl From<CreateVideoPlayCompleteEvent> for NewVideoPlayCompleteEvent {
    fn from(s: CreateVideoPlayCompleteEvent) -> Self {
        let id = Id::gen();
        let created_at = Local.timestamp_opt(s.timestamp / 1000, 0).unwrap();
        Self {
            id,
            reply_token: s.reply_token,
            delivery_context: NewDeliveryContext::from(s.delivery_context),
            video_play_complete: NewVideoPlayComplete::from(s.video_play_complete),
            mode: s.mode,
            webhook_event_id: s.webhook_event_id,
            created_at,
        }
    }
}

impl From<CreateVideoPlayComplete> for NewVideoPlayComplete {
    fn from(s: CreateVideoPlayComplete) -> Self {
        Self {
            tracking_id: s.tracking_id,
        }
    }
}

impl From<CreateMessageEvent> for NewMessageEvent {
    fn from(s: CreateMessageEvent) -> Self {
        let id = Id::gen();
        let created_at = Local.timestamp_opt(s.timestamp / 1000, 0).unwrap();
        Self {
            id,
            reply_token: s.reply_token,
            delivery_context: NewDeliveryContext::from(s.delivery_context),
            message: NewMessage::from(s.message),
            mode: s.mode,
            webhook_event_id: s.webhook_event_id,
            created_at,
        }
    }
}

impl From<CreateMessage> for NewMessage {
    fn from(s: CreateMessage) -> Self {
        match s {
            CreateMessage::Text(s) => NewMessage::Text(s.into()),
            CreateMessage::Image(s) => NewMessage::Image(s.into()),
            CreateMessage::Video(s) => NewMessage::Video(s.into()),
            CreateMessage::Audio(s) => NewMessage::Audio(s.into()),
            CreateMessage::File(s) => NewMessage::File(s.into()),
            CreateMessage::Location(s) => NewMessage::Location(s.into()),
            CreateMessage::Sticker(s) => NewMessage::Sticker(s.into()),
        }
    }
}

impl From<CreateTextMessage> for NewTextMessage {
    fn from(s: CreateTextMessage) -> Self {
        Self {
            id: s.id,
            text: s.text,
            emojis: s.emojis.into_iter().map(|e| e.into()).collect(),
        }
    }
}

impl From<CreateEmoji> for NewEmoji {
    fn from(s: CreateEmoji) -> Self {
        Self {
            index: s.index,
            length: s.length,
            product_id: s.product_id,
            emoji_id: s.emoji_id,
        }
    }
}

impl From<CreateImageMessage> for NewImageMessage {
    fn from(s: CreateImageMessage) -> Self {
        Self {
            id: s.id,
            content_provider: NewContentProvider::from(s.content_provider),
            image_set: s.image_set.map(|i| NewImageSet::from(i)),
        }
    }
}

impl From<CreateContentProvider> for NewContentProvider {
    fn from(s: CreateContentProvider) -> Self {
        match s {
            CreateContentProvider::Line => NewContentProvider::Line,
            CreateContentProvider::External(e) => NewContentProvider::External(e.into()),
        }
    }
}

impl From<CreateExternalContentProvider> for NewExternalContentProvider {
    fn from(s: CreateExternalContentProvider) -> Self {
        Self {
            original_content_url: s.original_content_url,
            preview_image_url: s.preview_image_url,
        }
    }
}

impl From<CreateImageSet> for NewImageSet {
    fn from(s: CreateImageSet) -> Self {
        Self {
            id: s.id,
            index: s.index,
            length: s.length,
        }
    }
}

impl From<CreateVideoMessage> for NewVideoMessage {
    fn from(s: CreateVideoMessage) -> Self {
        Self {
            id: s.id,
            duration: s.duration,
            content_provider: NewContentProvider::from(s.content_provider),
        }
    }
}

impl From<CreateAudioMessage> for NewAudioMessage {
    fn from(s: CreateAudioMessage) -> Self {
        Self {
            id: s.id,
            duration: s.duration,
            content_provider: NewContentProvider::from(s.content_provider),
        }
    }
}

impl From<CreateFileMessage> for NewFileMessage {
    fn from(s: CreateFileMessage) -> Self {
        Self {
            id: s.id,
            file_name: s.file_name,
            file_size: s.file_size,
        }
    }
}

impl From<CreateLocationMessage> for NewLocationMessage {
    fn from(s: CreateLocationMessage) -> Self {
        Self {
            id: s.id,
            title: s.title,
            address: s.address,
            latitude: Decimal::from_f64(s.latitude)
                .unwrap_or_else(|| panic!("Failed to convert Decimal {} to f64", s.latitude)),
            longitude: Decimal::from_f64(s.longitude)
                .unwrap_or_else(|| panic!("Failed to convert Decimal {} to f64", s.longitude)),
        }
    }
}

impl From<CreateStickerMessage> for NewStickerMessage {
    fn from(s: CreateStickerMessage) -> Self {
        Self {
            id: s.id,
            package_id: s.package_id,
            sticker_id: s.sticker_id,
            sticker_resource_type: NewStickerResourceType::from(s.sticker_resource_type),
            keywords: s.keywords,
            text: s.text,
        }
    }
}

impl From<CreateStickerResourceType> for NewStickerResourceType {
    fn from(s: CreateStickerResourceType) -> Self {
        match s {
            CreateStickerResourceType::Static => NewStickerResourceType::Static,
            CreateStickerResourceType::Animation => NewStickerResourceType::Animation,
            CreateStickerResourceType::Sound => NewStickerResourceType::Sound,
            CreateStickerResourceType::AnimationSound => NewStickerResourceType::AnimationSound,
            CreateStickerResourceType::Popup => NewStickerResourceType::Popup,
            CreateStickerResourceType::PopupSound => NewStickerResourceType::PopupSound,
            CreateStickerResourceType::Custom => NewStickerResourceType::Custom,
            CreateStickerResourceType::Message => NewStickerResourceType::Message,
        }
    }
}
