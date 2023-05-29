use derive_new::new;

pub enum UserEvent {
    Follow(UserFollowEvent),
    Unfollow(UserUnfollowEvent),
    Postback(UserPostbackEvent),
    VideoPlayComplete(UserVideoPlayCompleteEvent),
    Message(UserMessageEvent),
}

#[derive(new)]
pub struct UserFollowEvent {
    talk_room_id: String,
    reply_token: String,
    delivery_context: DeliveryContext,
    event_type: EventType,
    mode: String,
    timestamp: i64,
}

#[derive(new)]
pub struct UserUnfollowEvent {
    talk_room_id: String,
    reply_token: String,
    delivery_context: DeliveryContext,
    event_type: EventType,
    mode: String,
    timestamp: i64,
}

#[derive(new)]
pub struct UserPostbackEvent {
    talk_room_id: String,
    reply_token: String,
    delivery_context: DeliveryContext,
    event_type: EventType,
    postback: Postback,
    mode: String,
    timestamp: i64,
}

#[derive(new)]
pub struct UserVideoPlayCompleteEvent {
    talk_room_id: String,
    reply_token: String,
    delivery_context: DeliveryContext,
    event_type: EventType,
    video_play_complete: VideoPlayComplete,
    timestamp: i64,
}

pub struct UserMessageEvent {
    primary_user_id: PrimaryUserId,
    talk_room_id: String,
    reply_token: String,
    delivery_context: Option<DeliveryContext>,
    event_type: EventType,
    messages: Vec<Message>,
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

