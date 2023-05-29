use derive_new::new;

use super::{
    event_type::{DeliveryContext, EventType},
    primary_user_id::PrimaryUserId,
};

//
#[derive(new)]
pub struct UserMessage {
    primary_user_id: PrimaryUserId,
    talk_room_id: String,
    reply_token: String,
    delivery_context: Option<DeliveryContext>,
    event_type: EventType,
    messages: Vec<Message>,
    timestamp: i64,
}

#[derive(new)]
enum Message {
    Text(TextMessage),
    Image(ImageMessage),
    Video(VideoMessage),
    Audio(AudioMessage),
    File(FileMessage),
    Location(LocationMessage),
    Sticker(StickerMessage),
}

#[derive(new)]
struct TextMessage {
    id: String,
    text: String,
    emojis: Vec<Emoji>,
}

#[derive(new)]
struct Emoji {
    index: i32,
    length: i32,
    product_id: String,
    emoji_id: String,
}

#[derive(new)]
struct ImageMessage {
    id: String,
    content_provider: ContentProvider,
    image_set: LineWebhookImageSet,
}

#[derive(new)]
enum ContentProvider {
    Line,
    External {
        original_content_url: String,
        preview_image_url: Option<String>,
    },
}

#[derive(new)]
struct LineWebhookImageSet {
    id: String,
    index: i32,
    length: i32,
}

#[derive(new)]
struct VideoMessage {
    id: String,
    duration: i32,
    content_provider: ContentProvider,
}

#[derive(new)]
struct AudioMessage {
    id: String,
    duration: i32,
    content_provider: ContentProvider,
}

#[derive(new)]
struct FileMessage {
    id: String,
    file_name: String,
    file_size: i32,
}

#[derive(new)]
struct LocationMessage {
    id: String,
    title: String,
    address: String,
    latitude: f64,
    longitude: f64,
}

#[derive(new)]
struct StickerMessage {
    id: String,
    package_id: String,
    sticker_id: String,
    sticker_resource_type: StickerResourceType,
    keywords: Option<Vec<String>>,
    text: Option<String>,
}

#[derive(new)]
enum StickerResourceType {
    Static,
    Animated,
    Sound,
    AnimationSound,
    Popup,
    PupupSound,
    Custom,
    Message,
}
