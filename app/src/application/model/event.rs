use derive_new::new;

use super::line_user_auth::CreateLineUserAuth;

pub struct CreateUserEvent {
    pub create_line_user_auth: CreateLineUserAuth,
    pub create_user_event: CreateEvent,
}

pub enum CreateEvent {
    Follow(CreateFollowEvent),
    Unfollow(CreateUnfollowEvent),
    Postback(CreatePostbackEvent),
    VideoPlayComplete(CreateVideoPlayCompleteEvent),
    Message(CreateMessageEvent),
}

#[derive(new)]
pub struct CreateFollowEvent {
    pub reply_token: String,
    pub delivery_context: CreateDeliveryContext,
    pub mode: String,
    pub webhook_event_id: String,
    pub timestamp: i64,
}

#[derive(new)]
pub struct CreateUnfollowEvent {
    pub reply_token: String,
    pub delivery_context: CreateDeliveryContext,
    pub mode: String,
    pub webhook_event_id: String,
    pub timestamp: i64,
}

#[derive(new)]
pub struct CreatePostbackEvent {
    pub reply_token: String,
    pub delivery_context: CreateDeliveryContext,
    pub postback: CreatePostback,
    pub mode: String,
    pub webhook_event_id: String,
    pub timestamp: i64,
}

#[derive(new)]
pub struct CreateVideoPlayCompleteEvent {
    pub reply_token: String,
    pub delivery_context: CreateDeliveryContext,
    pub video_play_complete: CreateVideoPlayComplete,
    pub mode: String,
    pub webhook_event_id: String,
    pub timestamp: i64,
}

pub struct CreateMessageEvent {
    pub reply_token: String,
    pub delivery_context: CreateDeliveryContext,
    pub message: CreateMessage,
    pub mode: String,
    pub webhook_event_id: String,
    pub timestamp: i64,
}

#[derive(new)]
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

#[derive(new)]
pub struct CreatePostback {
    pub data: String,
    pub params: CreatePostbackParams,
}

#[derive(new)]
pub enum CreatePostbackParams {
    Datetime(CreatePostbackDatetimeParams),
    RichMenu(CreatePostbackRichMenuParams),
}

#[derive(new)]
pub struct CreatePostbackDatetimeParams {
    pub datetime: String,
}

#[derive(new)]
pub struct CreatePostbackRichMenuParams {
    pub new_rich_menu_alias_id: String,
    pub status: String,
}

#[derive(new)]
pub struct CreateVideoPlayComplete {
    pub tracking_id: String,
}

#[derive(new)]
pub enum CreateMessage {
    Text(CreateTextMessage),
    Image(CreateImageMessage),
    Video(CreateVideoMessage),
    Audio(CreateAudioMessage),
    File(CreateFileMessage),
    Location(CreateLocationMessage),
    Sticker(CreateStickerMessage),
}

#[derive(new)]
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

#[derive(new)]
pub struct CreateImageMessage {
    pub id: String,
    pub content_provider: CreateContentProvider,
    pub image_set: CreateImageSet,
}

#[derive(new)]
pub enum CreateContentProvider {
    Line,
    External {
        original_content_url: String,
        preview_image_url: Option<String>,
    },
}

#[derive(new)]
pub struct CreateImageSet {
    pub id: String,
    pub index: i32,
    pub length: i32,
}

#[derive(new)]
pub struct CreateVideoMessage {
    pub id: String,
    pub duration: i32,
    pub content_provider: CreateContentProvider,
}

#[derive(new)]
pub struct CreateAudioMessage {
    pub id: String,
    pub duration: i32,
    pub content_provider: CreateContentProvider,
}

#[derive(new)]
pub struct CreateFileMessage {
    pub id: String,
    pub file_name: String,
    pub file_size: i32,
}

#[derive(new)]
pub struct CreateLocationMessage {
    pub id: String,
    pub title: String,
    pub address: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(new)]
pub struct CreateStickerMessage {
    pub id: String,
    pub package_id: String,
    pub sticker_id: String,
    pub sticker_resource_type: CreateStickerResourceType,
    pub keywords: Option<Vec<String>>,
    pub text: Option<String>,
}

#[derive(new)]
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
