use crate::model::line_user_auth::CreateLineUserAuth;
use chrono::{Local, TimeZone};
use derive_new::new;

use domain::model::{
    message::event::{
        NewEvent, NewEventContentProvider, NewEventContentProviderExternal,
        NewEventDeliveryContext, NewEventEmoji, NewEventFollow, NewEventImageSet, NewEventMessage,
        NewEventMessageContent, NewEventMessageContentAudio, NewEventMessageContentFile,
        NewEventMessageContentImage, NewEventMessageContentLocation, NewEventMessageContentSticker,
        NewEventMessageContentText, NewEventMessageContentVideo, NewEventPostback,
        NewEventPostbackContent, NewEventPostbackParams, NewEventPostbackParamsDatetime,
        NewEventPostbackParamsRichMenu, NewEventStickerResourceType, NewEventUnfollow,
        NewEventVideoPlayComplete, NewEventVideoPlayCompleteContent,
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
    Follow(CreateEventFollow),
    Unfollow(CreateEventUnfollow),
    Postback(CreateEventPostback),
    VideoPlayComplete(CreateEventVideoPlayComplete),
    Message(CreateEventMessage),
}

#[derive(new, Clone)]
pub struct CreateEventFollow {
    pub reply_token: String,
    pub delivery_context: CreateEventDeliveryContext,
    pub mode: String,
    pub webhook_event_id: String,
    pub timestamp: i64,
}

#[derive(new, Clone)]
pub struct CreateEventUnfollow {
    pub delivery_context: CreateEventDeliveryContext,
    pub mode: String,
    pub webhook_event_id: String,
    pub timestamp: i64,
}

#[derive(new, Clone)]
pub struct CreateEventPostback {
    pub reply_token: String,
    pub delivery_context: CreateEventDeliveryContext,
    pub postback: CreateEventPostbackContent,
    pub mode: String,
    pub webhook_event_id: String,
    pub timestamp: i64,
}

#[derive(new, Clone)]
pub struct CreateEventVideoPlayComplete {
    pub reply_token: String,
    pub delivery_context: CreateEventDeliveryContext,
    pub video_play_complete: CreateEventVideoPlayCompleteContent,
    pub mode: String,
    pub webhook_event_id: String,
    pub timestamp: i64,
}

#[derive(new, Clone)]
pub struct CreateEventMessage {
    pub reply_token: String,
    pub delivery_context: CreateEventDeliveryContext,
    pub message: CreateEventMessageContent,
    pub mode: String,
    pub webhook_event_id: String,
    pub timestamp: i64,
}

#[derive(new, Debug, Clone)]
pub struct CreateEventDeliveryContext {
    pub is_redelivery: bool,
}

#[derive(new, Clone)]
pub struct CreateEventPostbackContent {
    pub data: String,
    pub params: CreateEventPostbackParams,
}

#[derive(new, Clone)]
pub enum CreateEventPostbackParams {
    Datetime(CreateEventPostbackParamsDatetime),
    RichMenu(CreateEventPostbackParamsRichMenu),
}

#[derive(new, Clone)]
pub enum CreateEventPostbackParamsDatetime {
    DateTime(String),
    Date(String),
    Time(String),
}

#[derive(new, Clone)]
pub struct CreateEventPostbackParamsRichMenu {
    pub new_rich_menu_alias_id: String,
    pub status: String,
}

#[derive(new, Clone)]
pub struct CreateEventVideoPlayCompleteContent {
    pub tracking_id: String,
}

#[derive(new, Clone)]
pub enum CreateEventMessageContent {
    Text(CreateEventMessageContentText),
    Image(CreateEventMessageContentImage),
    Video(CreateEventMessageContentVideo),
    Audio(CreateEventMessageContentAudio),
    File(CreateEventMessageContentFile),
    Location(CreateEventMessageContentLocation),
    Sticker(CreateEventMessageContentSticker),
}

#[derive(new, Clone)]
pub struct CreateEventMessageContentText {
    pub id: String,
    pub text: String,
    pub emojis: Vec<CreateEventEmoji>,
}

#[derive(new, Clone)]
pub struct CreateEventEmoji {
    pub index: i32,
    pub length: i32,
    pub product_id: String,
    pub emoji_id: String,
}

#[derive(new, Clone)]
pub struct CreateEventMessageContentImage {
    pub id: String,
    pub content_provider: CreateEventContentProvider,
    pub image_set: Option<CreateEventImageSet>,
}

#[derive(new, Clone)]
pub enum CreateEventContentProvider {
    Line,
    External(CreateEventContentProviderExternal),
}

#[derive(new, Clone)]
pub struct CreateEventContentProviderExternal {
    pub original_content_url: String,
    pub preview_image_url: Option<String>,
}

#[derive(new, Clone)]
pub struct CreateEventImageSet {
    pub id: String,
    pub index: i32,
    pub length: i32,
}

#[derive(new, Clone)]
pub struct CreateEventMessageContentVideo {
    pub id: String,
    pub duration: i32,
    pub content_provider: CreateEventContentProvider,
}

#[derive(new, Clone)]
pub struct CreateEventMessageContentAudio {
    pub id: String,
    pub duration: i32,
    pub content_provider: CreateEventContentProvider,
}

#[derive(new, Clone)]
pub struct CreateEventMessageContentFile {
    pub id: String,
    pub file_name: String,
    pub file_size: i32,
}

#[derive(new, Clone)]
pub struct CreateEventMessageContentLocation {
    pub id: String,
    pub title: String,
    pub address: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(new, Clone)]
pub struct CreateEventMessageContentSticker {
    pub id: String,
    pub package_id: String,
    pub sticker_id: String,
    pub sticker_resource_type: CreateEventStickerResourceType,
    pub keywords: Option<Vec<String>>,
    pub text: Option<String>,
}

#[derive(new, Clone)]
pub enum CreateEventStickerResourceType {
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

impl From<CreateEventDeliveryContext> for NewEventDeliveryContext {
    fn from(s: CreateEventDeliveryContext) -> Self {
        Self {
            is_redelivery: s.is_redelivery,
        }
    }
}

impl From<CreateEventFollow> for NewEventFollow {
    fn from(s: CreateEventFollow) -> Self {
        let id = Id::gen();
        let created_at = Local.timestamp_opt(s.timestamp / 1000, 0).unwrap();
        println!("created_at: {:?}", created_at);
        Self {
            id,
            reply_token: s.reply_token,
            delivery_context: NewEventDeliveryContext::from(s.delivery_context),
            mode: s.mode,
            webhook_event_id: s.webhook_event_id,
            created_at,
        }
    }
}

impl From<CreateEventUnfollow> for NewEventUnfollow {
    fn from(s: CreateEventUnfollow) -> Self {
        let id = Id::gen();
        let created_at = Local.timestamp_opt(s.timestamp / 1000, 0).unwrap();
        Self {
            id,
            delivery_context: NewEventDeliveryContext::from(s.delivery_context),
            mode: s.mode,
            webhook_event_id: s.webhook_event_id,
            created_at,
        }
    }
}

impl From<CreateEventPostback> for NewEventPostback {
    fn from(s: CreateEventPostback) -> Self {
        let id = Id::gen();
        let created_at = Local.timestamp_opt(s.timestamp / 1000, 0).unwrap();
        Self {
            id,
            reply_token: s.reply_token,
            delivery_context: NewEventDeliveryContext::from(s.delivery_context),
            postback: NewEventPostbackContent::from(s.postback),
            mode: s.mode,
            webhook_event_id: s.webhook_event_id,
            created_at,
        }
    }
}

impl From<CreateEventPostbackContent> for NewEventPostbackContent {
    fn from(s: CreateEventPostbackContent) -> Self {
        Self {
            data: s.data,
            params: NewEventPostbackParams::from(s.params),
        }
    }
}

impl From<CreateEventPostbackParams> for NewEventPostbackParams {
    fn from(s: CreateEventPostbackParams) -> Self {
        match s {
            CreateEventPostbackParams::Datetime(s) => {
                NewEventPostbackParams::Datetime(NewEventPostbackParamsDatetime::from(s))
            }
            CreateEventPostbackParams::RichMenu(s) => {
                NewEventPostbackParams::RichMenu(NewEventPostbackParamsRichMenu::from(s))
            }
        }
    }
}

impl From<CreateEventPostbackParamsDatetime> for NewEventPostbackParamsDatetime {
    fn from(s: CreateEventPostbackParamsDatetime) -> Self {
        match s {
            CreateEventPostbackParamsDatetime::DateTime(s) => {
                NewEventPostbackParamsDatetime::DateTime(s)
            }
            CreateEventPostbackParamsDatetime::Date(s) => NewEventPostbackParamsDatetime::Date(s),
            CreateEventPostbackParamsDatetime::Time(s) => NewEventPostbackParamsDatetime::Time(s),
        }
    }
}

impl From<CreateEventPostbackParamsRichMenu> for NewEventPostbackParamsRichMenu {
    fn from(s: CreateEventPostbackParamsRichMenu) -> Self {
        Self {
            new_rich_menu_alias_id: s.new_rich_menu_alias_id,
            status: s.status,
        }
    }
}

impl From<CreateEventVideoPlayComplete> for NewEventVideoPlayComplete {
    fn from(s: CreateEventVideoPlayComplete) -> Self {
        let id = Id::gen();
        let created_at = Local.timestamp_opt(s.timestamp / 1000, 0).unwrap();
        Self {
            id,
            reply_token: s.reply_token,
            delivery_context: NewEventDeliveryContext::from(s.delivery_context),
            video_play_complete: NewEventVideoPlayCompleteContent::from(s.video_play_complete),
            mode: s.mode,
            webhook_event_id: s.webhook_event_id,
            created_at,
        }
    }
}

impl From<CreateEventVideoPlayCompleteContent> for NewEventVideoPlayCompleteContent {
    fn from(s: CreateEventVideoPlayCompleteContent) -> Self {
        Self {
            tracking_id: s.tracking_id,
        }
    }
}

impl From<CreateEventMessage> for NewEventMessage {
    fn from(s: CreateEventMessage) -> Self {
        let id = Id::gen();
        let created_at = Local.timestamp_opt(s.timestamp / 1000, 0).unwrap();
        Self {
            id,
            reply_token: s.reply_token,
            delivery_context: NewEventDeliveryContext::from(s.delivery_context),
            message: NewEventMessageContent::from(s.message),
            mode: s.mode,
            webhook_event_id: s.webhook_event_id,
            created_at,
        }
    }
}

impl From<CreateEventMessageContent> for NewEventMessageContent {
    fn from(s: CreateEventMessageContent) -> Self {
        match s {
            CreateEventMessageContent::Text(s) => NewEventMessageContent::Text(s.into()),
            CreateEventMessageContent::Image(s) => NewEventMessageContent::Image(s.into()),
            CreateEventMessageContent::Video(s) => NewEventMessageContent::Video(s.into()),
            CreateEventMessageContent::Audio(s) => NewEventMessageContent::Audio(s.into()),
            CreateEventMessageContent::File(s) => NewEventMessageContent::File(s.into()),
            CreateEventMessageContent::Location(s) => NewEventMessageContent::Location(s.into()),
            CreateEventMessageContent::Sticker(s) => NewEventMessageContent::Sticker(s.into()),
        }
    }
}

impl From<CreateEventMessageContentText> for NewEventMessageContentText {
    fn from(s: CreateEventMessageContentText) -> Self {
        Self {
            id: s.id,
            text: s.text,
            emojis: s.emojis.into_iter().map(|e| e.into()).collect(),
        }
    }
}

impl From<CreateEventEmoji> for NewEventEmoji {
    fn from(s: CreateEventEmoji) -> Self {
        Self {
            index: s.index,
            length: s.length,
            product_id: s.product_id,
            emoji_id: s.emoji_id,
        }
    }
}

impl From<CreateEventMessageContentImage> for NewEventMessageContentImage {
    fn from(s: CreateEventMessageContentImage) -> Self {
        Self {
            id: s.id,
            content_provider: NewEventContentProvider::from(s.content_provider),
            image_set: s.image_set.map(|i| NewEventImageSet::from(i)),
        }
    }
}

impl From<CreateEventContentProvider> for NewEventContentProvider {
    fn from(s: CreateEventContentProvider) -> Self {
        match s {
            CreateEventContentProvider::Line => NewEventContentProvider::Line,
            CreateEventContentProvider::External(e) => NewEventContentProvider::External(e.into()),
        }
    }
}

impl From<CreateEventContentProviderExternal> for NewEventContentProviderExternal {
    fn from(s: CreateEventContentProviderExternal) -> Self {
        Self {
            original_content_url: s.original_content_url,
            preview_image_url: s.preview_image_url,
        }
    }
}

impl From<CreateEventImageSet> for NewEventImageSet {
    fn from(s: CreateEventImageSet) -> Self {
        Self {
            id: s.id,
            index: s.index,
            length: s.length,
        }
    }
}

impl From<CreateEventMessageContentVideo> for NewEventMessageContentVideo {
    fn from(s: CreateEventMessageContentVideo) -> Self {
        Self {
            id: s.id,
            duration: s.duration,
            content_provider: NewEventContentProvider::from(s.content_provider),
        }
    }
}

impl From<CreateEventMessageContentAudio> for NewEventMessageContentAudio {
    fn from(s: CreateEventMessageContentAudio) -> Self {
        Self {
            id: s.id,
            duration: s.duration,
            content_provider: NewEventContentProvider::from(s.content_provider),
        }
    }
}

impl From<CreateEventMessageContentFile> for NewEventMessageContentFile {
    fn from(s: CreateEventMessageContentFile) -> Self {
        Self {
            id: s.id,
            file_name: s.file_name,
            file_size: s.file_size,
        }
    }
}

impl From<CreateEventMessageContentLocation> for NewEventMessageContentLocation {
    fn from(s: CreateEventMessageContentLocation) -> Self {
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

impl From<CreateEventMessageContentSticker> for NewEventMessageContentSticker {
    fn from(s: CreateEventMessageContentSticker) -> Self {
        Self {
            id: s.id,
            package_id: s.package_id,
            sticker_id: s.sticker_id,
            sticker_resource_type: NewEventStickerResourceType::from(s.sticker_resource_type),
            keywords: s.keywords,
            text: s.text,
        }
    }
}

impl From<CreateEventStickerResourceType> for NewEventStickerResourceType {
    fn from(s: CreateEventStickerResourceType) -> Self {
        match s {
            CreateEventStickerResourceType::Static => NewEventStickerResourceType::Static,
            CreateEventStickerResourceType::Animation => NewEventStickerResourceType::Animation,
            CreateEventStickerResourceType::Sound => NewEventStickerResourceType::Sound,
            CreateEventStickerResourceType::AnimationSound => {
                NewEventStickerResourceType::AnimationSound
            }
            CreateEventStickerResourceType::Popup => NewEventStickerResourceType::Popup,
            CreateEventStickerResourceType::PopupSound => NewEventStickerResourceType::PopupSound,
            CreateEventStickerResourceType::Custom => NewEventStickerResourceType::Custom,
            CreateEventStickerResourceType::Message => NewEventStickerResourceType::Message,
        }
    }
}
