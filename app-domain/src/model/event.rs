use chrono::{DateTime, Local};
use derive_new::new;
use rust_decimal::Decimal;

use crate::model::Id;

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub enum Event {
    Follow(FollowEvent),
    Unfollow(UnfollowEvent),
    Postback(PostbackEvent),
    VideoPlayComplete(VideoPlayCompleteEvent),
    Message(MessageEvent),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FollowEvent {
    pub id: Id<Event>,
    pub reply_token: String,
    pub delivery_context: DeliveryContext,
    pub mode: String,
    pub webhook_event_id: String,
    pub created_at: DateTime<Local>,
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct UnfollowEvent {
    pub id: Id<Event>,
    pub reply_token: String,
    pub delivery_context: DeliveryContext,
    pub mode: String,
    pub webhook_event_id: String,
    pub created_at: DateTime<Local>,
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct PostbackEvent {
    pub id: Id<Event>,
    pub reply_token: String,
    pub delivery_context: DeliveryContext,
    pub postback: Postback,
    pub mode: String,
    pub webhook_event_id: String,
    pub created_at: DateTime<Local>,
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct VideoPlayCompleteEvent {
    pub id: Id<Event>,
    pub reply_token: String,
    pub delivery_context: DeliveryContext,
    pub video_play_complete: VideoPlayComplete,
    pub mode: String,
    pub webhook_event_id: String,
    pub created_at: DateTime<Local>,
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct MessageEvent {
    pub id: Id<Event>,
    pub reply_token: String,
    pub delivery_context: DeliveryContext,
    pub message: Message,
    pub mode: String,
    pub webhook_event_id: String,
    pub created_at: DateTime<Local>,
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub enum EventType {
    Message,
    Follow,
    Unfollow,
    Postback,
    VideoPlayComplete,
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct DeliveryContext {
    pub is_redelivery: bool,
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct Postback {
    pub data: String,
    pub params: PostbackParams,
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub enum PostbackParams {
    Datetime(PostbackDatetimeParams),
    RichMenu(PostbackRichMenuParams),
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct PostbackDatetimeParams {
    pub datetime: String,
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct PostbackRichMenuParams {
    pub new_rich_menu_alias_id: String,
    pub status: String,
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct VideoPlayComplete {
    pub tracking_id: String,
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub enum Message {
    Text(TextMessage),
    Image(ImageMessage),
    Video(VideoMessage),
    Audio(AudioMessage),
    File(FileMessage),
    Location(LocationMessage),
    Sticker(StickerMessage),
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct TextMessage {
    pub id: String,
    pub text: String,
    pub emojis: Vec<Emoji>,
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct Emoji {
    pub index: i32,
    pub length: i32,
    pub product_id: String,
    pub emoji_id: String,
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct ImageMessage {
    pub id: String,
    pub content_provider: ContentProvider,
    pub image_set: Option<ImageSet>,
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub enum ContentProvider {
    Line,
    External {
        original_content_url: String,
        preview_image_url: Option<String>,
    },
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct ImageSet {
    pub id: String,
    pub index: i32,
    pub length: i32,
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct VideoMessage {
    pub id: String,
    pub duration: i32,
    pub content_provider: ContentProvider,
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct AudioMessage {
    pub id: String,
    pub duration: i32,
    pub content_provider: ContentProvider,
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct FileMessage {
    pub id: String,
    pub file_name: String,
    pub file_size: i32,
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct LocationMessage {
    pub id: String,
    pub title: String,
    pub address: String,
    pub latitude: Decimal,
    pub longitude: Decimal,
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct StickerMessage {
    pub id: String,
    pub package_id: String,
    pub sticker_id: String,
    pub sticker_resource_type: StickerResourceType,
    pub keywords: Option<Vec<String>>,
    pub text: Option<String>,
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub enum StickerResourceType {
    Static,
    Animation,
    Sound,
    AnimationSound,
    Popup,
    PupupSound,
    Custom,
    Message,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NewEvent {
    Follow(NewFollowEvent),
    Unfollow(NewUnfollowEvent),
    Postback(NewPostbackEvent),
    VideoPlayComplete(NewVideoPlayCompleteEvent),
    Message(NewMessageEvent),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewFollowEvent {
    pub id: Id<Event>,
    pub reply_token: String,
    pub delivery_context: NewDeliveryContext,
    pub mode: String,
    pub webhook_event_id: String,
    pub created_at: DateTime<Local>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewUnfollowEvent {
    pub id: Id<Event>,
    pub reply_token: String,
    pub delivery_context: NewDeliveryContext,
    pub mode: String,
    pub webhook_event_id: String,
    pub created_at: DateTime<Local>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewPostbackEvent {
    pub id: Id<Event>,
    pub reply_token: String,
    pub delivery_context: NewDeliveryContext,
    pub postback: NewPostback,
    pub mode: String,
    pub webhook_event_id: String,
    pub created_at: DateTime<Local>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewVideoPlayCompleteEvent {
    pub id: Id<Event>,
    pub reply_token: String,
    pub delivery_context: NewDeliveryContext,
    pub video_play_complete: NewVideoPlayComplete,
    pub mode: String,
    pub webhook_event_id: String,
    pub created_at: DateTime<Local>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewMessageEvent {
    pub id: Id<Event>,
    pub reply_token: String,
    pub delivery_context: NewDeliveryContext,
    pub message: NewMessage,
    pub mode: String,
    pub webhook_event_id: String,
    pub created_at: DateTime<Local>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewDeliveryContext {
    pub is_redelivery: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewPostback {
    pub data: String,
    pub params: NewPostbackParams,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NewPostbackParams {
    Datetime(NewPostbackDatetimeParams),
    RichMenu(NewPostbackRichMenuParams),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewPostbackDatetimeParams {
    pub datetime: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewPostbackRichMenuParams {
    pub new_rich_menu_alias_id: String,
    pub status: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewVideoPlayComplete {
    pub tracking_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NewMessage {
    Text(NewTextMessage),
    Image(NewImageMessage),
    Video(NewVideoMessage),
    Audio(NewAudioMessage),
    File(NewFileMessage),
    Location(NewLocationMessage),
    Sticker(NewStickerMessage),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewTextMessage {
    pub id: String,
    pub text: String,
    pub emojis: Vec<NewEmoji>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewEmoji {
    pub index: i32,
    pub length: i32,
    pub product_id: String,
    pub emoji_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewImageMessage {
    pub id: String,
    pub content_provider: NewContentProvider,
    pub image_set: Option<NewImageSet>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NewContentProvider {
    Line,
    External {
        original_content_url: String,
        preview_image_url: Option<String>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewImageSet {
    pub id: String,
    pub index: i32,
    pub length: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewVideoMessage {
    pub id: String,
    pub duration: i32,
    pub content_provider: NewContentProvider,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewAudioMessage {
    pub id: String,
    pub duration: i32,
    pub content_provider: NewContentProvider,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewFileMessage {
    pub id: String,
    pub file_name: String,
    pub file_size: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewLocationMessage {
    pub id: String,
    pub title: String,
    pub address: String,
    pub latitude: Decimal,
    pub longitude: Decimal,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewStickerMessage {
    pub id: String,
    pub package_id: String,
    pub sticker_id: String,
    pub sticker_resource_type: NewStickerResourceType,
    pub keywords: Option<Vec<String>>,
    pub text: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NewStickerResourceType {
    Static,
    Animation,
    Sound,
    AnimationSound,
    Popup,
    PupupSound,
    Custom,
    Message,
}
