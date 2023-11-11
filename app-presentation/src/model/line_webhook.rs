use application::model::{
    event::{
        CreateEvent, CreateEventContentProvider, CreateEventContentProviderExternal,
        CreateEventDeliveryContext, CreateEventEmoji, CreateEventFollow, CreateEventImageSet,
        CreateEventMessage, CreateEventMessageContent, CreateEventMessageContentAudio,
        CreateEventMessageContentFile, CreateEventMessageContentImage,
        CreateEventMessageContentLocation, CreateEventMessageContentSticker,
        CreateEventMessageContentText, CreateEventMessageContentVideo, CreateEventPostback,
        CreateEventPostbackContent, CreateEventPostbackParams, CreateEventPostbackParamsDatetime,
        CreateEventPostbackParamsRichMenu, CreateEventStickerResourceType, CreateEventUnfollow,
        CreateEventVideoPlayComplete, CreateEventVideoPlayCompleteContent, CreateUserEvent,
    },
    line_user_auth::CreateLineUserAuth,
};
use derive_new::new;
use serde::{Deserialize, Serialize};
use strum_macros::Display;
use strum_macros::EnumString;
use validator::Validate;

#[cfg(test)]
use fake::Dummy;

#[derive(new, Serialize, Deserialize, Debug, Validate, Clone)]
pub struct LineWebhookEventRequests {
    pub destination: String,
    pub events: Vec<LineWebhookEvent>,
}

#[derive(new, Debug, Validate, Clone)]
pub struct LineWebhookEventRequest {
    pub destination: String,
    pub event: LineWebhookEvent,
}

impl LineWebhookEventRequest {
    pub fn user_id(&self) -> &String {
        self.event.user_id()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Display)]
#[cfg_attr(test, derive(Dummy))]
#[serde(tag = "type")]
pub enum LineWebhookEvent {
    #[serde(rename(deserialize = "follow"))]
    Follow(LineWebhookEventFollow),
    #[serde(rename(deserialize = "unfollow"))]
    Unfollow(LineWebhookEventUnfollow),
    #[serde(rename(deserialize = "postback"))]
    Postback(LineWebhookEventPostback),
    #[serde(rename(deserialize = "videoPlayComplete"))]
    VideoPlayComplete(LineWebhookEventVideoPlayComplete),
    #[serde(rename(deserialize = "message"))]
    Message(LineWebhookEventMessage),
}

impl LineWebhookEvent {
    pub fn user_id(&self) -> &String {
        match &self {
            LineWebhookEvent::Follow(e) => e.user_id(),
            LineWebhookEvent::Unfollow(e) => e.user_id(),
            LineWebhookEvent::Postback(e) => e.user_id(),
            LineWebhookEvent::VideoPlayComplete(e) => e.user_id(),
            LineWebhookEvent::Message(e) => e.user_id(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
#[cfg_attr(test, derive(Dummy))]
pub struct LineWebhookEventFollow {
    #[serde(rename(deserialize = "replyToken"))]
    reply_token: String,
    mode: String,
    #[cfg_attr(test, dummy(faker = "chrono::Local::now().timestamp()"))]
    timestamp: i64,
    source: LineWebhookEventSource,
    #[serde(rename(deserialize = "webhookEventId"))]
    webhook_event_id: String,
    #[serde(rename(deserialize = "deliveryContext"))]
    delivery_context: LineWebhookEventDeliveryContext,
}

impl LineWebhookEventFollow {
    pub fn user_id(&self) -> &String {
        &self.source.user_id()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
#[cfg_attr(test, derive(Dummy))]
pub struct LineWebhookEventUnfollow {
    mode: String,
    timestamp: i64,
    source: LineWebhookEventSource,
    #[serde(rename(deserialize = "webhookEventId"))]
    webhook_event_id: String,
    #[serde(rename(deserialize = "deliveryContext"))]
    delivery_context: LineWebhookEventDeliveryContext,
}

impl LineWebhookEventUnfollow {
    pub fn user_id(&self) -> &String {
        &self.source.user_id()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
#[cfg_attr(test, derive(Dummy))]
pub struct LineWebhookEventPostback {
    #[serde(rename(deserialize = "replyToken"))]
    reply_token: String,
    mode: String,
    timestamp: i64,
    source: LineWebhookEventSource,
    #[serde(rename(deserialize = "webhookEventId"))]
    webhook_event_id: String,
    #[serde(rename(deserialize = "deliveryContext"))]
    delivery_context: LineWebhookEventDeliveryContext,
    postback: LineWebhookEventPostbackContent,
}

impl LineWebhookEventPostback {
    pub fn user_id(&self) -> &String {
        &self.source.user_id()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
struct LineWebhookEventPostbackContent {
    data: String,
    params: Option<LineWebhookEventPostbackParams>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
#[serde(untagged)] // JSONにタグ名を含まない
enum LineWebhookEventPostbackParams {
    Datetime(LineWebhookEventPostbackParamsDatetime),
    RichMenu(LineWebhookEventPostbackParamsRichMenu),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
enum LineWebhookEventPostbackParamsDatetime {
    #[serde(rename(deserialize = "datetime"))]
    DateTime(String),
    #[serde(rename(deserialize = "date"))]
    Date(String),
    #[serde(rename(deserialize = "time"))]
    Time(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
struct LineWebhookEventPostbackParamsRichMenu {
    #[serde(rename(deserialize = "newRichMenuAliasId"))]
    new_rich_menu_alias_id: String,
    status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
#[cfg_attr(test, derive(Dummy))]
pub struct LineWebhookEventVideoPlayComplete {
    #[serde(rename(deserialize = "replyToken"))]
    reply_token: String,
    mode: String,
    timestamp: i64,
    source: LineWebhookEventSource,
    #[serde(rename(deserialize = "webhookEventId"))]
    webhook_event_id: String,
    #[serde(rename(deserialize = "deliveryContext"))]
    delivery_context: LineWebhookEventDeliveryContext,
    #[serde(rename(deserialize = "videoPlayComplete"))]
    video_play_complete: Option<LineWebhookVideoPlayCompleteContent>,
}

impl LineWebhookEventVideoPlayComplete {
    pub fn user_id(&self) -> &String {
        &self.source.user_id()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
struct LineWebhookVideoPlayCompleteContent {
    #[serde(rename(deserialize = "trackingId"))]
    tracking_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
#[cfg_attr(test, derive(Dummy))]
pub struct LineWebhookEventMessage {
    #[serde(rename(deserialize = "replyToken"))]
    reply_token: String,
    mode: String,
    timestamp: i64,
    source: LineWebhookEventSource,
    #[serde(rename(deserialize = "webhookEventId"))]
    webhook_event_id: String,
    #[serde(rename(deserialize = "deliveryContext"))]
    delivery_context: LineWebhookEventDeliveryContext,
    message: LineWebhookEventMessageContent,
}

impl LineWebhookEventMessage {
    pub fn user_id(&self) -> &String {
        &self.source.user_id()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Display)]
#[cfg_attr(test, derive(Dummy))]
#[serde(tag = "type")]
pub enum LineWebhookEventSource {
    #[serde(rename(deserialize = "user"))]
    User(LineWebhookEventSourceUser),
    #[serde(rename(deserialize = "group"))]
    Group(LineWebhookEventSourceGroup),
    #[serde(rename(deserialize = "room"))]
    Room(LineWebhookEventSourceRoom),
}

impl LineWebhookEventSource {
    pub fn user_id(&self) -> &String {
        match &self {
            LineWebhookEventSource::User(s) => &s.user_id,
            LineWebhookEventSource::Group(s) => &s.user_id,
            LineWebhookEventSource::Room(s) => &s.user_id,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
pub struct LineWebhookEventSourceUser {
    #[serde(rename(deserialize = "userId"))]
    user_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
pub struct LineWebhookEventSourceGroup {
    #[serde(rename(deserialize = "groupId"))]
    group_id: String,
    #[serde(rename(deserialize = "userId"))]
    user_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
pub struct LineWebhookEventSourceRoom {
    #[serde(rename(deserialize = "roomId"))]
    room_id: String,
    #[serde(rename(deserialize = "userId"))]
    user_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
struct LineWebhookEventDeliveryContext {
    #[serde(rename(deserialize = "isRedelivery"))]
    is_redelivery: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
#[serde(tag = "type")] // JSONにtypeというフィールドでタグ名を含む
enum LineWebhookEventMessageContent {
    #[serde(rename(deserialize = "text"))]
    Text(LineWebhookEventMessageContentText),
    #[serde(rename(deserialize = "image"))]
    Image(LineWebhookEventMessageContentImage),
    #[serde(rename(deserialize = "video"))]
    Video(LineWebhookEventMessageContentVideo),
    #[serde(rename(deserialize = "audio"))]
    Audio(LineWebhookEventMessageContentAudio),
    #[serde(rename(deserialize = "file"))]
    File(LineWebhookEventMessageContentFile),
    #[serde(rename(deserialize = "location"))]
    Location(LineWebhookEventMessageContentLocation),
    #[serde(rename(deserialize = "sticker"))]
    Sticker(LineWebhookEventMessageContentSticker),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
struct LineWebhookEventMessageContentText {
    id: String,
    text: String,
    emojis: Vec<LineWebhookEventEmoji>,
    mention: Option<LineWebhookEventMention>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
struct LineWebhookEventEmoji {
    index: i32,
    length: i32,
    #[serde(rename(deserialize = "productId"))]
    product_id: String,
    #[serde(rename(deserialize = "emojiId"))]
    emoji_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
struct LineWebhookEventMention {
    mentionees: Vec<LineWebhookEventMentionee>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
#[serde(tag = "type")]
enum LineWebhookEventMentionee {
    #[serde(rename(deserialize = "user"))]
    LineWebhookEventMentioneeUser,
    #[serde(rename(deserialize = "all"))]
    LineWebhookEventMentioneeAll,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
struct LineWebhookEventMentioneeUser {
    index: i32,
    length: i32,
    #[serde(rename(deserialize = "userId"))]
    user_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
struct LineWebhookEventMentioneeAll {
    index: i32,
    length: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
struct LineWebhookEventMessageContentImage {
    id: String,
    #[serde(rename(deserialize = "contentProvider"))]
    content_provider: LineWebhookEventContentProvider,
    image_set: Option<LineWebhookEventImageSet>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
#[serde(tag = "type")]
enum LineWebhookEventContentProvider {
    #[serde(rename(deserialize = "line"))]
    Line,
    #[serde(rename(deserialize = "external"))]
    External(LineWebhookEventContentProviderExternal),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
struct LineWebhookEventContentProviderExternal {
    #[serde(rename(deserialize = "originalContentUrl"))]
    original_content_url: String,
    #[serde(rename(deserialize = "previewImageUrl"))]
    preview_image_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
struct LineWebhookEventImageSet {
    id: String,
    index: i32,
    length: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
struct LineWebhookEventMessageContentVideo {
    id: String,
    duration: i32,
    #[serde(rename(deserialize = "contentProvider"))]
    content_provider: LineWebhookEventContentProvider,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
struct LineWebhookEventMessageContentAudio {
    id: String,
    duration: i32,
    #[serde(rename(deserialize = "contentProvider"))]
    content_provider: LineWebhookEventContentProvider,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
struct LineWebhookEventMessageContentFile {
    id: String,
    #[serde(rename(deserialize = "fileName"))]
    file_name: String,
    #[serde(rename(deserialize = "fileSize"))]
    file_size: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
struct LineWebhookEventMessageContentLocation {
    id: String,
    title: String,
    address: String,
    latitude: f64,
    longitude: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Dummy))]
struct LineWebhookEventMessageContentSticker {
    id: String,
    #[serde(rename(deserialize = "packageId"))]
    package_id: String,
    #[serde(rename(deserialize = "stickerId"))]
    sticker_id: String,
    #[serde(rename(deserialize = "stickerResourceType"))]
    sticker_resource_type: LineWebhookEventStickerResourceType,
    keywords: Option<Vec<String>>,
    text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, EnumString)]
#[cfg_attr(test, derive(Dummy))]
enum LineWebhookEventStickerResourceType {
    #[serde(rename(deserialize = "STATIC"))]
    Static,
    #[serde(rename(deserialize = "ANIMATION"))]
    Animation,
    #[serde(rename(deserialize = "SOUND"))]
    Sound,
    #[serde(rename(deserialize = "ANIMATION_SOUND"))]
    AnimationSound,
    #[serde(rename(deserialize = "POPUP"))]
    Popup,
    #[serde(rename(deserialize = "POPUP_SOUND"))]
    PopupSound,
    #[serde(rename(deserialize = "CUSTOM"))]
    Custom,
    #[serde(rename(deserialize = "MESSAGE"))]
    Message,
}

impl From<LineWebhookEventRequests> for Vec<LineWebhookEventRequest> {
    fn from(r: LineWebhookEventRequests) -> Self {
        r.events
            .iter()
            .map(|e: &LineWebhookEvent| LineWebhookEventRequest {
                destination: r.destination.clone(),
                event: e.clone(),
            })
            .collect()
    }
}

impl From<LineWebhookEventRequest> for CreateUserEvent {
    fn from(r: LineWebhookEventRequest) -> Self {
        let event = r.event.clone();
        let create_event = match event {
            LineWebhookEvent::Follow(s) => CreateEvent::Follow(s.into()),
            LineWebhookEvent::Unfollow(s) => CreateEvent::Unfollow(s.into()),
            LineWebhookEvent::Postback(s) => CreateEvent::Postback(s.into()),
            LineWebhookEvent::VideoPlayComplete(s) => CreateEvent::VideoPlayComplete(s.into()),
            LineWebhookEvent::Message(s) => CreateEvent::Message(s.into()),
        };
        Self {
            create_line_user_auth: CreateLineUserAuth {
                user_id: r.user_id().clone(),
            },
            create_event,
        }
    }
}

impl From<LineWebhookEventFollow> for CreateEventFollow {
    fn from(s: LineWebhookEventFollow) -> Self {
        Self {
            reply_token: s.reply_token,
            delivery_context: CreateEventDeliveryContext {
                is_redelivery: s.delivery_context.is_redelivery,
            },
            mode: s.mode,
            webhook_event_id: s.webhook_event_id,
            timestamp: s.timestamp,
        }
    }
}

impl From<LineWebhookEventUnfollow> for CreateEventUnfollow {
    fn from(s: LineWebhookEventUnfollow) -> Self {
        Self {
            delivery_context: CreateEventDeliveryContext {
                is_redelivery: s.delivery_context.is_redelivery,
            },
            mode: s.mode,
            webhook_event_id: s.webhook_event_id,
            timestamp: s.timestamp,
        }
    }
}

impl From<LineWebhookEventPostback> for CreateEventPostback {
    fn from(s: LineWebhookEventPostback) -> Self {
        Self {
            reply_token: s.reply_token,
            delivery_context: CreateEventDeliveryContext {
                is_redelivery: s.delivery_context.is_redelivery,
            },
            postback: CreateEventPostbackContent {
                data: s.postback.clone().data,
                params: s.postback.clone().params.unwrap().into(),
            },
            mode: s.mode,
            webhook_event_id: s.webhook_event_id,
            timestamp: s.timestamp,
        }
    }
}

impl From<LineWebhookEventPostbackParams> for CreateEventPostbackParams {
    fn from(s: LineWebhookEventPostbackParams) -> Self {
        match s {
            LineWebhookEventPostbackParams::Datetime(p) => {
                CreateEventPostbackParams::Datetime(p.into())
            }
            LineWebhookEventPostbackParams::RichMenu(p) => {
                CreateEventPostbackParams::RichMenu(p.into())
            }
        }
    }
}

impl From<LineWebhookEventPostbackParamsDatetime> for CreateEventPostbackParamsDatetime {
    fn from(s: LineWebhookEventPostbackParamsDatetime) -> Self {
        match s {
            LineWebhookEventPostbackParamsDatetime::DateTime(datetime) => {
                CreateEventPostbackParamsDatetime::DateTime(datetime)
            }
            LineWebhookEventPostbackParamsDatetime::Date(date) => {
                CreateEventPostbackParamsDatetime::Date(date)
            }
            LineWebhookEventPostbackParamsDatetime::Time(time) => {
                CreateEventPostbackParamsDatetime::Time(time)
            }
        }
    }
}

impl From<LineWebhookEventPostbackParamsRichMenu> for CreateEventPostbackParamsRichMenu {
    fn from(s: LineWebhookEventPostbackParamsRichMenu) -> Self {
        Self {
            new_rich_menu_alias_id: s.new_rich_menu_alias_id,
            status: s.status,
        }
    }
}

impl From<LineWebhookEventVideoPlayComplete> for CreateEventVideoPlayComplete {
    fn from(s: LineWebhookEventVideoPlayComplete) -> Self {
        Self {
            reply_token: s.reply_token,
            delivery_context: CreateEventDeliveryContext {
                is_redelivery: s.delivery_context.is_redelivery,
            },
            video_play_complete: CreateEventVideoPlayCompleteContent {
                tracking_id: s.video_play_complete.unwrap().tracking_id,
            },
            mode: s.mode,
            webhook_event_id: s.webhook_event_id,
            timestamp: s.timestamp,
        }
    }
}

impl From<LineWebhookEventMessage> for CreateEventMessage {
    fn from(s: LineWebhookEventMessage) -> Self {
        Self {
            reply_token: s.reply_token,
            delivery_context: CreateEventDeliveryContext {
                is_redelivery: s.delivery_context.is_redelivery,
            },
            message: s.message.into(),
            mode: s.mode,
            webhook_event_id: s.webhook_event_id,
            timestamp: s.timestamp,
        }
    }
}

impl From<LineWebhookEventMessageContent> for CreateEventMessageContent {
    fn from(s: LineWebhookEventMessageContent) -> Self {
        match s {
            LineWebhookEventMessageContent::Text(m) => CreateEventMessageContent::Text(m.into()),
            LineWebhookEventMessageContent::Image(m) => CreateEventMessageContent::Image(m.into()),
            LineWebhookEventMessageContent::Video(m) => CreateEventMessageContent::Video(m.into()),
            LineWebhookEventMessageContent::Audio(m) => CreateEventMessageContent::Audio(m.into()),
            LineWebhookEventMessageContent::File(m) => CreateEventMessageContent::File(m.into()),
            LineWebhookEventMessageContent::Location(m) => {
                CreateEventMessageContent::Location(m.into())
            }
            LineWebhookEventMessageContent::Sticker(m) => {
                CreateEventMessageContent::Sticker(m.into())
            }
        }
    }
}

impl From<LineWebhookEventMessageContentText> for CreateEventMessageContentText {
    fn from(s: LineWebhookEventMessageContentText) -> Self {
        Self {
            id: s.id,
            text: s.text,
            emojis: s
                .emojis
                .iter()
                .map(|e| CreateEventEmoji {
                    index: e.index,
                    length: e.length,
                    product_id: e.product_id.clone(),
                    emoji_id: e.emoji_id.clone(),
                })
                .collect(),
        }
    }
}

impl From<LineWebhookEventMessageContentImage> for CreateEventMessageContentImage {
    fn from(s: LineWebhookEventMessageContentImage) -> Self {
        Self {
            id: s.id,
            content_provider: s.content_provider.into(),
            image_set: s.image_set.map(|i| i.into()),
        }
    }
}

impl From<LineWebhookEventContentProvider> for CreateEventContentProvider {
    fn from(value: LineWebhookEventContentProvider) -> Self {
        match value {
            LineWebhookEventContentProvider::Line => CreateEventContentProvider::Line,
            LineWebhookEventContentProvider::External(e) => {
                CreateEventContentProvider::External(e.into())
            }
        }
    }
}

impl From<LineWebhookEventContentProviderExternal> for CreateEventContentProviderExternal {
    fn from(s: LineWebhookEventContentProviderExternal) -> Self {
        Self {
            original_content_url: s.original_content_url,
            preview_image_url: s.preview_image_url,
        }
    }
}

impl From<LineWebhookEventImageSet> for CreateEventImageSet {
    fn from(s: LineWebhookEventImageSet) -> Self {
        Self {
            id: s.id,
            index: s.index,
            length: s.length,
        }
    }
}

impl From<LineWebhookEventMessageContentVideo> for CreateEventMessageContentVideo {
    fn from(s: LineWebhookEventMessageContentVideo) -> Self {
        Self {
            id: s.id,
            duration: s.duration,
            content_provider: s.content_provider.into(),
        }
    }
}

impl From<LineWebhookEventMessageContentAudio> for CreateEventMessageContentAudio {
    fn from(s: LineWebhookEventMessageContentAudio) -> Self {
        Self {
            id: s.id,
            duration: s.duration,
            content_provider: s.content_provider.into(),
        }
    }
}

impl From<LineWebhookEventMessageContentFile> for CreateEventMessageContentFile {
    fn from(s: LineWebhookEventMessageContentFile) -> Self {
        Self {
            id: s.id,
            file_name: s.file_name,
            file_size: s.file_size,
        }
    }
}

impl From<LineWebhookEventMessageContentLocation> for CreateEventMessageContentLocation {
    fn from(s: LineWebhookEventMessageContentLocation) -> Self {
        Self {
            id: s.id,
            title: s.title,
            address: s.address,
            latitude: s.latitude,
            longitude: s.longitude,
        }
    }
}

impl From<LineWebhookEventMessageContentSticker> for CreateEventMessageContentSticker {
    fn from(s: LineWebhookEventMessageContentSticker) -> Self {
        Self {
            id: s.id,
            package_id: s.package_id,
            sticker_id: s.sticker_id,
            sticker_resource_type: s.sticker_resource_type.into(),
            keywords: s.keywords,
            text: s.text,
        }
    }
}

impl From<LineWebhookEventStickerResourceType> for CreateEventStickerResourceType {
    fn from(s: LineWebhookEventStickerResourceType) -> Self {
        match s {
            LineWebhookEventStickerResourceType::Static => CreateEventStickerResourceType::Static,
            LineWebhookEventStickerResourceType::Animation => {
                CreateEventStickerResourceType::Animation
            }
            LineWebhookEventStickerResourceType::Sound => CreateEventStickerResourceType::Sound,
            LineWebhookEventStickerResourceType::AnimationSound => {
                CreateEventStickerResourceType::AnimationSound
            }
            LineWebhookEventStickerResourceType::Popup => CreateEventStickerResourceType::Popup,
            LineWebhookEventStickerResourceType::PopupSound => {
                CreateEventStickerResourceType::PopupSound
            }
            LineWebhookEventStickerResourceType::Custom => CreateEventStickerResourceType::Custom,
            LineWebhookEventStickerResourceType::Message => CreateEventStickerResourceType::Message,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
     * follow event
     */
    #[test]
    fn test_line_webhook_follow_event() {
        let destination = "line_id".to_string();
        let json = r#"
        {
            "replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA",
            "type": "follow",
            "mode": "active",
            "timestamp": 1462629479859,
            "source": {
                "type": "user",
                "userId": "U00000000000000000000000000000000"
            },
            "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
            "deliveryContext": {
                "isRedelivery": false
            }
        }
        "#;
        let line_webhook_event: LineWebhookEvent =
            serde_json::from_str(json).expect("Failed to deserialize");
        println!("line_webhook_event{:?}", line_webhook_event);
        LineWebhookEventRequest::new(destination, line_webhook_event);
    }
    /*
     * unfollow event
     */
    #[test]
    fn test_line_webhook_unfollow_event() {
        let destination = "line_id".to_string();
        let json = r#"
        {
            "type": "unfollow",
            "mode": "active",
            "timestamp": 1462629479859,
            "source": {
                "type": "user",
                "userId": "U00000000000000000000000000000000"
                },
            "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
            "deliveryContext": {
                "isRedelivery": false
            }
        }
        "#;
        let line_webhook_event: LineWebhookEvent =
            serde_json::from_str(json).expect("Failed to deserialize");
        LineWebhookEventRequest::new(destination, line_webhook_event);
    }
    /*
     * unfollow event
     */
    #[test]
    fn test_line_webhook_video_play_complete_event() {
        let destination = "line_id".to_string();
        let json = r#"
        {
            "replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA",
            "type": "videoPlayComplete",
            "mode": "active",
            "timestamp": 1462629479859,
            "source": {
                "type": "user",
                "userId": "U00000000000000000000000000000000"
            },
            "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
            "deliveryContext": {
                "isRedelivery": false
            },
            "videoPlayComplete": {
                "trackingId": "track-id"
            }
        }
        "#;
        let line_webhook_event: LineWebhookEvent =
            serde_json::from_str(json).expect("Failed to deserialize");
        LineWebhookEventRequest::new(destination, line_webhook_event);
    }
    /*
     * message event
     */
    #[test]
    fn test_line_webhook_message_event() {
        let destination = "line_id".to_string();
        /*
         * text
         */
        let json = r#"
        {
            "replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA",
            "type": "message",
            "mode": "active",
            "timestamp": 1462629479859,
            "source": {
                "type": "user",
                "userId": "U00000000000000000000000000000000"
            },
            "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
            "deliveryContext": {
                "isRedelivery": false
            },
            "message": {
                "id": "444573844083572737",
                "type": "text",
                "quoteToken": "q3Plxr4AgKd...",
                "text": "@All @example Good Morning!! (love)",
                "emojis": [
                    {
                        "index": 29,
                        "length": 6,
                        "productId": "5ac1bfd5040ab15980c9b435",
                        "emojiId": "001"
                    }
                ],
                "mention": {
                    "mentionees": [
                        {
                            "index": 0,
                            "length": 4,
                            "type": "all"
                        },
                        {
                            "index": 5,
                            "length": 8,
                            "userId": "U49585cd0d5...",
                            "type": "user"
                        }
                    ]
                }
            }
        }
        "#;
        let line_webhook_event: LineWebhookEvent =
            serde_json::from_str(json).expect("Failed to deserialize");
        LineWebhookEventRequest::new(destination.clone(), line_webhook_event);

        /*
         * image 1
         */
        let json = r#"
        {
            "type": "message",
            "message": {
                "type": "image",
                "id": "354718705033693859",
                "quoteToken": "q3Plxr4AgKd...",
                "contentProvider": {
                    "type": "line"
                },
                "imageSet": {
                    "id": "E005D41A7288F41B65593ED38FF6E9834B046AB36A37921A56BC236F13A91855",
                    "index": 1,
                    "total": 2
                }
            },
            "timestamp": 1627356924513,
            "source": {
                "type": "user",
                "userId": "U4af4980629..."
            },
            "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
            "deliveryContext": {
                "isRedelivery": false
            },
            "replyToken": "7840b71058e24a5d91f9b5726c7512c9",
            "mode": "active"
        }
        "#;
        let line_webhook_event: LineWebhookEvent =
            serde_json::from_str(json).expect("Failed to deserialize");
        LineWebhookEventRequest::new(destination.clone(), line_webhook_event);
        /*
         * image 2
         * ドキュメントを元に自作
         */
        let json = r#"
        {
            "type": "message",
            "message": {
                "type": "image",
                "id": "354718705033693861",
                "quoteToken": "yHAz4Ua2wx7...",
                "contentProvider": {
                    "type": "external",
                    "originalContentUrl": "https://example.com",
                    "previewImageUrl": "https://example.com"
                },
                "imageSet": {
                    "id": "E005D41A7288F41B65593ED38FF6E9834B046AB36A37921A56BC236F13A91855",
                    "index": 2,
                    "total": 2
                }
            },
            "timestamp": 1627356924722,
            "source": {
                "type": "user",
                "userId": "U4af4980629..."
            },
            "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
            "deliveryContext": {
                "isRedelivery": false
            },
            "replyToken": "fbf94e269485410da6b7e3a5e33283e8",
            "mode": "active"
        }
        "#;
        let line_webhook_event: LineWebhookEvent =
            serde_json::from_str(json).expect("Failed to deserialize");
        LineWebhookEventRequest::new(destination.clone(), line_webhook_event);
        /*
         * video
         */
        let json = r#"
        {
            "replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA",
            "type": "message",
            "mode": "active",
            "timestamp": 1462629479859,
            "source": {
                "type": "user",
                "userId": "U4af4980629..."
            },
            "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
            "deliveryContext": {
                "isRedelivery": false
            },
            "message": {
                "id": "325708",
                "type": "video",
                "quoteToken": "q3Plxr4AgKd...",
                "duration": 60000,
                "contentProvider": {
                    "type": "external",
                    "originalContentUrl": "https://example.com/original.mp4",
                    "previewImageUrl": "https://example.com/preview.jpg"
                }
            }
        }
        "#;
        let line_webhook_event: LineWebhookEvent =
            serde_json::from_str(json).expect("Failed to deserialize");
        LineWebhookEventRequest::new(destination.clone(), line_webhook_event);
        /*
         * audio
         */
        let json = r#"
        {
            "replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA",
            "type": "message",
            "mode": "active",
            "timestamp": 1462629479859,
            "source": {
                "type": "user",
                "userId": "U4af4980629..."
            },
            "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
            "deliveryContext": {
                "isRedelivery": false
            },
            "message": {
                "id": "325708",
                "type": "audio",
                "duration": 60000,
                "contentProvider": {
                    "type": "line"
                }
            }
        }
        "#;
        let line_webhook_event: LineWebhookEvent =
            serde_json::from_str(json).expect("Failed to deserialize");
        LineWebhookEventRequest::new(destination.clone(), line_webhook_event);
        /*
         * file
         */
        let json = r#"
        {
            "replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA",
            "type": "message",
            "mode": "active",
            "timestamp": 1462629479859,
            "source": {
                "type": "user",
                "userId": "U4af4980629..."
            },
            "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
            "deliveryContext": {
                "isRedelivery": false
            },
            "message": {
                "id": "325708",
                "type": "file",
                "fileName": "file.txt",
                "fileSize": 2138
            }
        }
        "#;
        let line_webhook_event: LineWebhookEvent =
            serde_json::from_str(json).expect("Failed to deserialize");
        LineWebhookEventRequest::new(destination.clone(), line_webhook_event);
        /*
         * location
         */
        let json = r#"
        {
            "replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA",
            "type": "message",
            "mode": "active",
            "timestamp": 1462629479859,
            "source": {
                "type": "user",
                "userId": "U4af4980629..."
            },
            "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
            "deliveryContext": {
                "isRedelivery": false
            },
            "message": {
                "id": "325708",
                "type": "location",
                "title": "my location",
                "address": "日本、〒102-8282 東京都千代田区紀尾井町1番3号",
                "latitude": 35.67966,
                "longitude": 139.73669
            }
        }
        "#;
        let line_webhook_event: LineWebhookEvent =
            serde_json::from_str(json).expect("Failed to deserialize");
        LineWebhookEventRequest::new(destination.clone(), line_webhook_event);
        /*
         * sticker
         * アニメーションスタンプの例
         */
        let json = r#"
        {
            "replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA",
            "type": "message",
            "mode": "active",
            "timestamp": 1462629479859,
            "source": {
                "type": "user",
                "userId": "U4af4980629..."
            },
            "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
            "deliveryContext": {
                "isRedelivery": false
            },
            "message": {
                "type": "sticker",
                "id": "1501597916",
                "quoteToken": "q3Plxr4AgKd...",
                "stickerId": "52002738",
                "packageId": "11537",
                "stickerResourceType": "ANIMATION",
                "keywords": [
                    "cony",
                    "sally",
                    "Staring",
                    "hi",
                    "whatsup",
                    "line",
                    "howdy",
                    "HEY",
                    "Peeking",
                    "wave",
                    "peek",
                    "Hello",
                    "yo",
                    "greetings"
                ]
            }
        }
        "#;
        let line_webhook_event: LineWebhookEvent =
            serde_json::from_str(json).expect("Failed to deserialize");
        LineWebhookEventRequest::new(destination.clone(), line_webhook_event);
        /*
         * sticker
         * メッセージスタンプの例
         */
        let json = r#"
        {
            "replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA",
            "type": "message",
            "mode": "active",
            "timestamp": 1462629479859,
            "source": {
                "type": "user",
                "userId": "U4af4980629..."
            },
            "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
            "deliveryContext": {
                "isRedelivery": false
            },
            "message": {
                "type": "sticker",
                "id": "123456789012345678",
                "quoteToken": "q3Plxr4AgKd...",
                "stickerId": "738839",
                "packageId": "12287",
                "stickerResourceType": "MESSAGE",
                "keywords": [
                    "Anticipation",
                    "Sparkle",
                    "Straight face",
                    "Staring",
                    "Thinking"
                ],
                "text": "今週末\n一緒に\n遊ぼうよ！"
            }
        }
        "#;
        let line_webhook_event: LineWebhookEvent =
            serde_json::from_str(json).expect("Failed to deserialize");
        println!("line_webhook_event{:?}", line_webhook_event);
        LineWebhookEventRequest::new(destination.clone(), line_webhook_event);
        /*
         * postback
         * 日時選択アクションのポストバックイベントの場合
         */
        let json = r#"
        {
            "replyToken": "b60d432864f44d079f6d8efe86cf404b",
            "type": "postback",
            "mode": "active",
            "source": {
                "userId": "U91eeaf62d...",
                "type": "user"
            },
            "timestamp": 1513669370317,
            "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
            "deliveryContext": {
                "isRedelivery": false
            },
            "postback": {
                "data": "storeId=12345",
                "params": {
                    "datetime": "2017-12-25T01:00"
                }
            }
        }
        "#;
        let line_webhook_event: LineWebhookEvent =
            serde_json::from_str(json).expect("Failed to deserialize");
        LineWebhookEventRequest::new(destination, line_webhook_event);
    }
    /*
     * postback event
     */
    #[test]
    fn test_line_webhook_postback_event() {
        let destination = "line_id".to_string();
        /*
         * postback
         * 日時選択アクションのポストバックイベントの場合
         */
        let json = r#"
        {
            "replyToken": "b60d432864f44d079f6d8efe86cf404b",
            "type": "postback",
            "mode": "active",
            "source": {
                "userId": "U91eeaf62d...",
                "type": "user"
            },
            "timestamp": 1513669370317,
            "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
            "deliveryContext": {
                "isRedelivery": false
            },
            "postback": {
                "data": "storeId=12345",
                "params": {
                    "datetime": "2017-12-25T01:00"
                }
            }
        }
        "#;
        let line_webhook_event: LineWebhookEvent =
            serde_json::from_str(json).expect("Failed to deserialize");
        LineWebhookEventRequest::new(destination.clone(), line_webhook_event);
        /*
         * postback
         * リッチメニュー切替アクションのポストバックイベントの場合
         */
        let json = r#"
        {
            "replyToken": "b60d432864f44d079f6d8efe86cf404b",
            "type": "postback",
            "mode": "active",
            "source": {
                "userId": "U91eeaf62d...",
                "type": "user"
            },
            "timestamp": 1619754620404,
            "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
            "deliveryContext": {
                "isRedelivery": false
            },
            "postback": {
                "data": "richmenu-changed-to-b",
                "params": {
                    "newRichMenuAliasId": "richmenu-alias-b",
                    "status": "SUCCESS"
                }
            }
        }
        "#;
        let line_webhook_event: LineWebhookEvent =
            serde_json::from_str(json).expect("Failed to deserialize");
        LineWebhookEventRequest::new(destination.clone(), line_webhook_event);
        /*
         * postback
         * ボタンを押されたときのポストバックイベントの場合
         */
        let json = r#"
        {
            "replyToken": "b60d432864f44d079f6d8efe86cf404b",
            "type": "postback",
            "mode": "active",
            "source": {
                "userId": "U91eeaf62d...",
                "type": "user"
            },
            "timestamp": 1619754620404,
            "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
            "deliveryContext": {
                "isRedelivery": false
            },
            "postback": {
                "data": "richmenu-changed-to-b"
            }
        }
        "#;
        let line_webhook_event: LineWebhookEvent =
            serde_json::from_str(json).expect("Failed to deserialize");
        LineWebhookEventRequest::new(destination, line_webhook_event);
    }
}
