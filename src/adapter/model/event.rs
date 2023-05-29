use chrono::{DateTime, Local};
use serde::Serialize;
use strum_macros::Display;

#[derive(Serialize)]
pub struct EventTable {
    #[serde(rename(serialize = "documentId"))]
    document_id: String,
    #[serde(rename(serialize = "talkRoomId"))]
    talk_room_id: String,
    #[serde(rename(serialize = "replyToken"))]
    reply_token: Option<String>,
    #[serde(rename(serialize = "webhookEventId"))]
    webhook_event_id: Option<String>,
    #[serde(rename(serialize = "DeliveryContextTable"))]
    delivery_context: Option<String>,
    #[serde(rename(serialize = "communicationType"))]
    communication_type: CommunicationTypeTable,
    #[serde(rename(serialize = "sendingType"))]
    sending_type: SendingTypeTable,
    #[serde(rename(serialize = "sendingMethod"))]
    sending_method: SendingMethod,
    sender: Option<Sender>,
    #[serde(rename(serialize = "eventType"))]
    event_type: Option<EventTypeTable>,
    messages: Option<Vec<MessageTable>>,
    postback: Option<PostbackTable>,
    video_play_complete: Option<VideoPlayCompleteTable>,
    #[serde(rename(serialize = "createdAt"))]
    created_at: DateTime<Local>,
    #[serde(rename(serialize = "updatedAt"))]
    updated_at: DateTime<Local>,
}

#[derive(Serialize)]
pub struct DeliveryContextTable {
    is_redelivery: bool,
}

#[derive(Serialize, Display)]
pub enum CommunicationTypeTable {
    #[strum(serialize = "send")]
    Send,
    #[strum(serialize = "receive")]
    Receive,
}

#[derive(Serialize, Display)]
pub enum SendingTypeTable {
    #[strum(serialize = "manual")]
    Manual,
    #[strum(serialize = "bot")]
    Bot,
}

#[derive(Serialize, Display)]
pub enum SendingMethod {
    #[strum(serialize = "replay")]
    Reply,
    #[strum(serialize = "push")]
    Push,
}

#[derive(Serialize)]
pub struct Sender {
    id: i64,
    name: String,
    picture_url: String,
    email: String,
    sender_role: SenderRoleTable,
}

#[derive(Serialize, Display)]
enum SenderRoleTable {
    #[strum(serialize = "sender")]
    Sender,
}

#[derive(Serialize, Display)]
pub enum EventTypeTable {
    #[strum(serialize = "message")]
    Message,
    #[strum(serialize = "follow")]
    Follow,
    #[strum(serialize = "unfollow")]
    Unfollow,
    #[strum(serialize = "postback")]
    Postback,
    #[strum(serialize = "videoPlayComplete")]
    VideoPlayComplete,
}

#[derive(Serialize)]
pub struct PostbackTable {
    pub data: String,
    pub params: PostbackParamsTable,
}

#[derive(Serialize)]
pub enum PostbackParamsTable {
    Datetime(PostbackDatetimeParamsTable),
    RichMenu(PostbackRichMenuParamsTable),
}

#[derive(Serialize)]
pub struct PostbackDatetimeParamsTable {
    pub datetime: String,
}

#[derive(Serialize)]
pub struct PostbackRichMenuParamsTable {
    #[serde(rename(serialize = "newRichMenuAliasId"))]
    pub new_rich_menu_alias_id: String,
    pub status: String,
}

#[derive(Serialize)]
pub struct VideoPlayCompleteTable {
    #[serde(rename(serialize = "trackingId"))]
    pub tracking_id: String,
}

#[derive(Serialize, Display)]
#[serde(tag = "messageType")] // JSONにmessageTypeというフィールドでタグ名を含む
pub enum MessageTable {
    #[strum(serialize = "text")]
    Text(TextMessageTable),
    #[strum(serialize = "image")]
    Image(ImageMessageTable),
    #[strum(serialize = "video")]
    Video(VideoMessageTable),
    #[strum(serialize = "audio")]
    Audio(AudioMessageTable),
    #[strum(serialize = "file")]
    File(FileMessageTable),
    #[strum(serialize = "location")]
    Location(LocationMessageTable),
    #[strum(serialize = "sticker")]
    Sticker(StickerMessageTable),
}

#[derive(Serialize)]
pub struct TextMessageTable {
    pub id: String,
    pub text: String,
    pub emojis: Vec<EmojiTable>,
}

#[derive(Serialize)]
pub struct EmojiTable {
    pub index: i32,
    pub length: i32,
    #[serde(rename(serialize = "productId"))]
    pub product_id: String,
    #[serde(rename(serialize = "emojiId"))]
    pub emoji_id: String,
}

#[derive(Serialize)]
pub struct ImageMessageTable {
    pub id: String,
    #[serde(rename(serialize = "contentProvider"))]
    pub content_provider: ContentProviderTable,
    #[serde(rename(serialize = "imageSet"))]
    pub image_set: ImageSetTable,
}

#[derive(Serialize, Display)]
#[serde(tag = "type")]
pub enum ContentProviderTable {
    #[strum(serialize = "line")]
    Line,
    #[strum(serialize = "external")]
    External {
        #[serde(rename(serialize = "originalContentUrl"))]
        original_content_url: String,
        #[serde(rename(serialize = "previewImageUrl"))]
        preview_image_url: Option<String>,
    },
}

#[derive(Serialize)]
pub struct ImageSetTable {
    pub id: String,
    pub index: i32,
    pub length: i32,
}

#[derive(Serialize)]
pub struct VideoMessageTable {
    pub id: String,
    pub duration: i32,
    #[serde(rename(serialize = "contentProvider"))]
    pub content_provider: ContentProviderTable,
}

#[derive(Serialize)]
pub struct AudioMessageTable {
    pub id: String,
    pub duration: i32,
    #[serde(rename(serialize = "contentProvider"))]
    pub content_provider: ContentProviderTable,
}

#[derive(Serialize)]
pub struct FileMessageTable {
    pub id: String,
    #[serde(rename(serialize = "fileName"))]
    pub file_name: String,
    #[serde(rename(serialize = "fileSize"))]
    pub file_size: i32,
}

#[derive(Serialize)]
pub struct LocationMessageTable {
    id: String,
    title: String,
    address: String,
    latitude: f64,
    longitude: f64,
}

#[derive(Serialize)]
pub struct StickerMessageTable {
    id: String,
    #[serde(rename(serialize = "packageId"))]
    package_id: String,
    #[serde(rename(serialize = "stickerId"))]
    sticker_id: String,
    #[serde(rename(serialize = "stickerResourceType"))]
    sticker_resource_type: StickerResourceTypeTable,
    keywords: Option<Vec<String>>,
    text: Option<String>,
}

#[derive(Serialize, Display)]
pub enum StickerResourceTypeTable {
    #[strum(serialize = "STATIC")]
    Static,
    #[strum(serialize = "ANIMATION")]
    Animation,
    #[strum(serialize = "SOUND")]
    Sound,
    #[strum(serialize = "ANIMATION_SOUND")]
    AnimationSound,
    #[strum(serialize = "POPUP")]
    Popup,
    #[strum(serialize = "POPUP_SOUND")]
    PupupSound,
    #[strum(serialize = "CUSTOM")]
    Custom,
    #[strum(serialize = "MESSAGE")]
    Message,
}
