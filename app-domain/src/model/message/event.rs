use chrono::{DateTime, Local};
use derive_new::new;
use rust_decimal::Decimal;

use crate::model::Id;

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub enum Event {
    Follow(EventFollow),
    Unfollow(EventUnfollow),
    Postback(EventPostback),
    VideoPlayComplete(EventVideoPlayComplete),
    Message(EventMessage),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventFollow {
    pub id: Id<Event>,
    pub reply_token: String,
    pub delivery_context: EventDeliveryContext,
    pub mode: String,
    pub webhook_event_id: String,
    pub created_at: DateTime<Local>,
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct EventUnfollow {
    pub id: Id<Event>,
    pub delivery_context: EventDeliveryContext,
    pub mode: String,
    pub webhook_event_id: String,
    pub created_at: DateTime<Local>,
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct EventPostback {
    pub id: Id<Event>,
    pub reply_token: String,
    pub delivery_context: EventDeliveryContext,
    pub postback: EventPostbackContent,
    pub mode: String,
    pub webhook_event_id: String,
    pub created_at: DateTime<Local>,
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct EventVideoPlayComplete {
    pub id: Id<Event>,
    pub reply_token: String,
    pub delivery_context: EventDeliveryContext,
    pub video_play_complete: EventVideoPlayCompleteContent,
    pub mode: String,
    pub webhook_event_id: String,
    pub created_at: DateTime<Local>,
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct EventMessage {
    pub id: Id<Event>,
    pub reply_token: String,
    pub delivery_context: EventDeliveryContext,
    pub message: EventMessageContent,
    pub mode: String,
    pub webhook_event_id: String,
    pub created_at: DateTime<Local>,
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct EventDeliveryContext {
    pub is_redelivery: bool,
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct EventPostbackContent {
    pub data: String,
    pub params: EventPostbackParams,
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub enum EventPostbackParams {
    Datetime(EventPostbackParamsDatetime),
    RichMenu(EventPostbackParamsRichMenu),
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub enum EventPostbackParamsDatetime {
    DateTime(String),
    Date(String),
    Time(String),
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct EventPostbackParamsRichMenu {
    pub new_rich_menu_alias_id: String,
    pub status: String,
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct EventVideoPlayCompleteContent {
    pub tracking_id: String,
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub enum EventMessageContent {
    Text(EventMessageContentText),
    Image(EventMessageContentImage),
    Video(EventMessageContentVideo),
    Audio(EventMessageContentAudio),
    File(EventMessageContentFile),
    Location(EventMessageContentLocation),
    Sticker(EventMessageContentSticker),
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct EventMessageContentText {
    pub id: String,
    pub text: String,
    pub emojis: Vec<EventEmoji>,
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct EventEmoji {
    pub index: i32,
    pub length: i32,
    pub product_id: String,
    pub emoji_id: String,
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct EventMessageContentImage {
    pub id: String,
    pub content_provider: EventContentProvider,
    pub image_set: Option<EventImageSet>,
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub enum EventContentProvider {
    Line,
    External(EventContentProviderExternal),
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct EventContentProviderExternal {
    original_content_url: String,
    preview_image_url: Option<String>,
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct EventImageSet {
    pub id: String,
    pub index: i32,
    pub length: i32,
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct EventMessageContentVideo {
    pub id: String,
    pub duration: i32,
    pub content_provider: EventContentProvider,
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct EventMessageContentAudio {
    pub id: String,
    pub duration: i32,
    pub content_provider: EventContentProvider,
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct EventMessageContentFile {
    pub id: String,
    pub file_name: String,
    pub file_size: i32,
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct EventMessageContentLocation {
    pub id: String,
    pub title: String,
    pub address: String,
    pub latitude: Decimal,
    pub longitude: Decimal,
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct EventMessageContentSticker {
    pub id: String,
    pub package_id: String,
    pub sticker_id: String,
    pub sticker_resource_type: EventStickerResourceType,
    pub keywords: Option<Vec<String>>,
    pub text: Option<String>,
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub enum EventStickerResourceType {
    Static,
    Animation,
    Sound,
    AnimationSound,
    Popup,
    PopupSound,
    Custom,
    Message,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NewEvent {
    Follow(NewEventFollow),
    Unfollow(NewEventUnfollow),
    Postback(NewEventPostback),
    VideoPlayComplete(NewEventVideoPlayComplete),
    Message(NewEventMessage),
}

impl NewEvent {
    pub fn id(&self) -> &Id<Event> {
        match self {
            NewEvent::Follow(e) => &e.id,
            NewEvent::Unfollow(e) => &e.id,
            NewEvent::Postback(e) => &e.id,
            NewEvent::VideoPlayComplete(e) => &e.id,
            NewEvent::Message(e) => &e.id,
        }
    }
    pub fn created_at(&self) -> &DateTime<Local> {
        match self {
            NewEvent::Follow(e) => &e.created_at,
            NewEvent::Unfollow(e) => &e.created_at,
            NewEvent::Postback(e) => &e.created_at,
            NewEvent::VideoPlayComplete(e) => &e.created_at,
            NewEvent::Message(e) => &e.created_at,
        }
    }
    pub fn follow(&self) -> bool {
        match self {
            NewEvent::Unfollow(_) => false,
            _ => true,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewEventFollow {
    pub id: Id<Event>,
    pub reply_token: String,
    pub delivery_context: NewEventDeliveryContext,
    pub mode: String,
    pub webhook_event_id: String,
    pub created_at: DateTime<Local>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewEventUnfollow {
    pub id: Id<Event>,
    pub delivery_context: NewEventDeliveryContext,
    pub mode: String,
    pub webhook_event_id: String,
    pub created_at: DateTime<Local>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewEventPostback {
    pub id: Id<Event>,
    pub reply_token: String,
    pub delivery_context: NewEventDeliveryContext,
    pub postback: NewEventPostbackContent,
    pub mode: String,
    pub webhook_event_id: String,
    pub created_at: DateTime<Local>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewEventVideoPlayComplete {
    pub id: Id<Event>,
    pub reply_token: String,
    pub delivery_context: NewEventDeliveryContext,
    pub video_play_complete: NewEventVideoPlayCompleteContent,
    pub mode: String,
    pub webhook_event_id: String,
    pub created_at: DateTime<Local>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewEventMessage {
    pub id: Id<Event>,
    pub reply_token: String,
    pub delivery_context: NewEventDeliveryContext,
    pub message: NewEventMessageContent,
    pub mode: String,
    pub webhook_event_id: String,
    pub created_at: DateTime<Local>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewEventDeliveryContext {
    pub is_redelivery: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewEventPostbackContent {
    pub data: String,
    pub params: NewEventPostbackParams,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NewEventPostbackParams {
    Datetime(NewEventPostbackParamsDatetime),
    RichMenu(NewEventPostbackParamsRichMenu),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NewEventPostbackParamsDatetime {
    DateTime(String),
    Date(String),
    Time(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewEventPostbackParamsRichMenu {
    pub new_rich_menu_alias_id: String,
    pub status: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewEventVideoPlayCompleteContent {
    pub tracking_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NewEventMessageContent {
    Text(NewEventMessageContentText),
    Image(NewEventMessageContentImage),
    Video(NewEventMessageContentVideo),
    Audio(NewEventMessageContentAudio),
    File(NewEventMessageContentFile),
    Location(NewEventMessageContentLocation),
    Sticker(NewEventMessageContentSticker),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewEventMessageContentText {
    pub id: String,
    pub text: String,
    pub emojis: Vec<NewEventEmoji>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewEventEmoji {
    pub index: i32,
    pub length: i32,
    pub product_id: String,
    pub emoji_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewEventMessageContentImage {
    pub id: String,
    pub content_provider: NewEventContentProvider,
    pub image_set: Option<NewEventImageSet>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NewEventContentProvider {
    Line,
    External(NewEventContentProviderExternal),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewEventContentProviderExternal {
    pub original_content_url: String,
    pub preview_image_url: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewEventImageSet {
    pub id: String,
    pub index: i32,
    pub length: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewEventMessageContentVideo {
    pub id: String,
    pub duration: i32,
    pub content_provider: NewEventContentProvider,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewEventMessageContentAudio {
    pub id: String,
    pub duration: i32,
    pub content_provider: NewEventContentProvider,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewEventMessageContentFile {
    pub id: String,
    pub file_name: String,
    pub file_size: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewEventMessageContentLocation {
    pub id: String,
    pub title: String,
    pub address: String,
    pub latitude: Decimal,
    pub longitude: Decimal,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewEventMessageContentSticker {
    pub id: String,
    pub package_id: String,
    pub sticker_id: String,
    pub sticker_resource_type: NewEventStickerResourceType,
    pub keywords: Option<Vec<String>>,
    pub text: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NewEventStickerResourceType {
    Static,
    Animation,
    Sound,
    AnimationSound,
    Popup,
    PopupSound,
    Custom,
    Message,
}
