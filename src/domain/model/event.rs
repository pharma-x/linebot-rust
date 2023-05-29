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
    talk_room_id: String,
    reply_token: String,
    delivery_context: DeliveryContext,
    event_type: EventType,
    mode: String,
    webhook_event_id: String,
    timestamp: i64,
}

#[derive(new)]
pub struct UnfollowEvent {
    talk_room_id: String,
    reply_token: String,
    delivery_context: DeliveryContext,
    event_type: EventType,
    mode: String,
    webhook_event_id: String,
    timestamp: i64,
}

#[derive(new)]
pub struct PostbackEvent {
    talk_room_id: String,
    reply_token: String,
    delivery_context: DeliveryContext,
    event_type: EventType,
    postback: Postback,
    mode: String,
    webhook_event_id: String,
    timestamp: i64,
}

#[derive(new)]
pub struct VideoPlayCompleteEvent {
    talk_room_id: String,
    reply_token: String,
    delivery_context: DeliveryContext,
    event_type: EventType,
    video_play_complete: VideoPlayComplete,
    mode: String,
    webhook_event_id: String,
    timestamp: i64,
}

pub struct MessageEvent {
    talk_room_id: String,
    reply_token: String,
    delivery_context: DeliveryContext,
    event_type: EventType,
    message: Message,
    mode: String,
    webhook_event_id: String,
    timestamp: i64,
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
    pub id: String,
    pub text: String,
    pub emojis: Vec<Emoji>,
}

#[derive(new)]
pub struct Emoji {
    pub index: i32,
    pub length: i32,
    pub product_id: String,
    pub emoji_id: String,
}

#[derive(new)]
pub struct ImageMessage {
    pub id: String,
    pub content_provider: ContentProvider,
    pub image_set: ImageSet,
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
    pub id: String,
    pub index: i32,
    pub length: i32,
}

#[derive(new)]
pub struct VideoMessage {
    pub id: String,
    pub duration: i32,
    pub content_provider: ContentProvider,
}

#[derive(new)]
pub struct AudioMessage {
    pub id: String,
    pub duration: i32,
    pub content_provider: ContentProvider,
}

#[derive(new)]
pub struct FileMessage {
    pub id: String,
    pub file_name: String,
    pub file_size: i32,
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
    Animation,
    Sound,
    AnimationSound,
    Popup,
    PupupSound,
    Custom,
    Message,
}
