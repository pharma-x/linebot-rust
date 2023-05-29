use derive_new::new;

use crate::domain::model::{talk_room::TalkRoom, user_event::UserEvent};

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
    pub delivery_context: DeliveryContext,
    pub event_type: EventType,
    pub mode: String,
    pub timestamp: i64,
}

#[derive(new)]
pub struct CreateUnfollowEvent {
    pub reply_token: String,
    pub delivery_context: DeliveryContext,
    pub event_type: EventType,
    pub mode: String,
    pub timestamp: i64,
}

#[derive(new)]
pub struct CreatePostbackEvent {
    pub reply_token: String,
    pub delivery_context: DeliveryContext,
    pub event_type: EventType,
    pub postback: Postback,
    pub mode: String,
    pub timestamp: i64,
}

#[derive(new)]
pub struct CreateVideoPlayCompleteEvent {
    pub reply_token: String,
    pub delivery_context: DeliveryContext,
    pub event_type: EventType,
    pub video_play_complete: VideoPlayComplete,
    pub mode: String,
    pub timestamp: i64,
}

pub struct CreateMessageEvent {
    pub reply_token: String,
    pub delivery_context: DeliveryContext,
    pub event_type: EventType,
    pub message: Message,
    pub mode: String,
    pub timestamp: i64,
}

#[derive(new)]
pub enum EventType {
    Message,
    Follow,
    Unfollow,
    Postback,
    VideoPlayComplete,
}

#[derive(new, Debug, Clone)]
pub struct DeliveryContext {
    is_redelivery: bool,
}

#[derive(new)]
pub struct Postback {
    data: String,
    params: PostbackParams,
}

#[derive(new)]
pub enum PostbackParams {
    Datetime(PostbackDatetimeParams),
    RichMenu(PostbackRichMenuParams),
}

#[derive(new)]
pub struct PostbackDatetimeParams {
    datetime: String,
}

#[derive(new)]
pub struct PostbackRichMenuParams {
    new_rich_menu_alias_id: String,
    status: String,
}

#[derive(new)]
pub struct VideoPlayComplete {
    tracking_id: String,
}

#[derive(new)]
pub enum Message {
    Text(TextMessage),
    Image(ImageMessage),
    Video(VideoMessage),
    Audio(AudioMessage),
    File(FileMessage),
    Location(LocationMessage),
    Sticker(StickerMessage),
}

#[derive(new)]
pub struct TextMessage {
    id: String,
    text: String,
    emojis: Vec<Emoji>,
}

#[derive(new)]
pub struct Emoji {
    index: i32,
    length: i32,
    product_id: String,
    emoji_id: String,
}

#[derive(new)]
pub struct ImageMessage {
    id: String,
    content_provider: ContentProvider,
    image_set: ImageSet,
}

#[derive(new)]
pub enum ContentProvider {
    Line,
    External {
        original_content_url: String,
        preview_image_url: Option<String>,
    },
}

#[derive(new)]
pub struct ImageSet {
    id: String,
    index: i32,
    length: i32,
}

#[derive(new)]
pub struct VideoMessage {
    id: String,
    duration: i32,
    content_provider: ContentProvider,
}

#[derive(new)]
pub struct AudioMessage {
    id: String,
    duration: i32,
    content_provider: ContentProvider,
}

#[derive(new)]
pub struct FileMessage {
    id: String,
    file_name: String,
    file_size: i32,
}

#[derive(new)]
pub struct LocationMessage {
    id: String,
    title: String,
    address: String,
    latitude: f64,
    longitude: f64,
}

#[derive(new)]
pub struct StickerMessage {
    id: String,
    package_id: String,
    sticker_id: String,
    sticker_resource_type: StickerResourceType,
    keywords: Option<Vec<String>>,
    text: Option<String>,
}

#[derive(new)]
pub enum StickerResourceType {
    Static,
    Animated,
    Sound,
    AnimationSound,
    Popup,
    PupupSound,
    Custom,
    Message,
}

impl From<CreateUserEvent> for UserEvent {
    fn from(s: CreateUserEvent) -> Self {
        todo!()
    }
}
