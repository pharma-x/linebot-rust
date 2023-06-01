use derive_new::new;

pub enum Event {
    Follow(FollowEvent),
    Unfollow(UnfollowEvent),
    Postback(PostbackEvent),
    VideoPlayComplete(VideoPlayCompleteEvent),
    Message(MessageEvent),
}

#[derive(new)]
pub struct FollowEvent {
    pub id: String,
    pub talk_room_id: String,
    pub reply_token: String,
    pub delivery_context: DeliveryContext,
    pub mode: String,
    pub webhook_event_id: String,
    pub timestamp: i64,
}

#[derive(new)]
pub struct UnfollowEvent {
    pub id: String,
    pub talk_room_id: String,
    pub reply_token: String,
    pub delivery_context: DeliveryContext,
    pub mode: String,
    pub webhook_event_id: String,
    pub timestamp: i64,
}

#[derive(new)]
pub struct PostbackEvent {
    pub id: String,
    pub talk_room_id: String,
    pub reply_token: String,
    pub delivery_context: DeliveryContext,
    pub postback: Postback,
    pub mode: String,
    pub webhook_event_id: String,
    pub timestamp: i64,
}

#[derive(new)]
pub struct VideoPlayCompleteEvent {
    pub id: String,
    pub talk_room_id: String,
    pub reply_token: String,
    pub delivery_context: DeliveryContext,
    pub video_play_complete: VideoPlayComplete,
    pub mode: String,
    pub webhook_event_id: String,
    pub timestamp: i64,
}

pub struct MessageEvent {
    pub id: String,
    pub talk_room_id: String,
    pub reply_token: String,
    pub delivery_context: DeliveryContext,
    pub message: Message,
    pub mode: String,
    pub webhook_event_id: String,
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
    pub is_redelivery: bool,
}

#[derive(new)]
pub struct Postback {
    pub data: String,
    pub params: PostbackParams,
}

#[derive(new)]
pub enum PostbackParams {
    Datetime(PostbackDatetimeParams),
    RichMenu(PostbackRichMenuParams),
}

#[derive(new)]
pub struct PostbackDatetimeParams {
    pub datetime: String,
}

#[derive(new)]
pub struct PostbackRichMenuParams {
    pub new_rich_menu_alias_id: String,
    pub status: String,
}

#[derive(new)]
pub struct VideoPlayComplete {
    pub tracking_id: String,
}

#[derive(Clone)]
pub enum Message {
    Text(TextMessage),
    Image(ImageMessage),
    Video(VideoMessage),
    Audio(AudioMessage),
    File(FileMessage),
    Location(LocationMessage),
    Sticker(StickerMessage),
}

#[derive(new, Clone)]
pub struct TextMessage {
    pub id: String,
    pub text: String,
    pub emojis: Vec<Emoji>,
}

#[derive(new, Clone)]
pub struct Emoji {
    pub index: i32,
    pub length: i32,
    pub product_id: String,
    pub emoji_id: String,
}

#[derive(new, Clone)]
pub struct ImageMessage {
    pub id: String,
    pub content_provider: ContentProvider,
    pub image_set: ImageSet,
}

#[derive(new, Clone)]
pub enum ContentProvider {
    Line,
    External {
        original_content_url: String,
        preview_image_url: Option<String>,
    },
}

#[derive(new, Clone)]
pub struct ImageSet {
    pub id: String,
    pub index: i32,
    pub length: i32,
}

#[derive(new, Clone)]
pub struct VideoMessage {
    pub id: String,
    pub duration: i32,
    pub content_provider: ContentProvider,
}

#[derive(new, Clone)]
pub struct AudioMessage {
    pub id: String,
    pub duration: i32,
    pub content_provider: ContentProvider,
}

#[derive(new, Clone)]
pub struct FileMessage {
    pub id: String,
    pub file_name: String,
    pub file_size: i32,
}

#[derive(new, Clone)]
pub struct LocationMessage {
    pub id: String,
    pub title: String,
    pub address: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(new, Clone)]
pub struct StickerMessage {
    pub id: String,
    pub package_id: String,
    pub sticker_id: String,
    pub sticker_resource_type: StickerResourceType,
    pub keywords: Option<Vec<String>>,
    pub text: Option<String>,
}

#[derive(new, Clone)]
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
