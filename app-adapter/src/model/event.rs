use chrono::{DateTime, Local};
use rust_decimal::{prelude::FromPrimitive, prelude::ToPrimitive, Decimal};
use serde::{Deserialize, Serialize};
use strum_macros::Display;

use domain::model::{
    event::{
        AudioMessage, ContentProvider, DeliveryContext, Emoji, Event, ExternalContentProvider,
        FileMessage, FollowEvent, ImageMessage, ImageSet, LocationMessage, Message, MessageEvent,
        NewAudioMessage, NewContentProvider, NewDeliveryContext, NewEmoji, NewEvent,
        NewExternalContentProvider, NewFileMessage, NewFollowEvent, NewImageMessage, NewImageSet,
        NewLocationMessage, NewMessage, NewMessageEvent, NewPostbackDatetimeParams,
        NewPostbackEvent, NewPostbackParams, NewPostbackRichMenuParams, NewStickerMessage,
        NewStickerResourceType, NewTextMessage, NewUnfollowEvent, NewVideoMessage,
        NewVideoPlayCompleteEvent, Postback, PostbackDatetimeParams, PostbackEvent, PostbackParams,
        PostbackRichMenuParams, StickerMessage, StickerResourceType, TextMessage, UnfollowEvent,
        VideoMessage, VideoPlayComplete, VideoPlayCompleteEvent,
    },
    Id,
};

#[derive(Serialize, Deserialize, Display, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum EventTable {
    Follow(FollowEventTable),
    Unfollow(UnfollowEventTable),
    Postback(PostbackEventTable),
    VideoPlayComplete(VideoPlayCompleteEventTable),
    Message(MessageEventTable),
}

impl EventTable {
    pub fn created_at(&self) -> DateTime<Local> {
        match self {
            EventTable::Follow(e) => e.created_at,
            EventTable::Unfollow(e) => e.created_at,
            EventTable::Message(e) => e.created_at,
            EventTable::Postback(e) => e.created_at,
            EventTable::VideoPlayComplete(e) => e.created_at,
        }
    }
    pub fn into_event(&self, document_id: String) -> Event {
        match &self {
            EventTable::Follow(f) => Event::Follow(f.into_event(document_id)),
            EventTable::Unfollow(u) => Event::Unfollow(u.into_event(document_id)),
            EventTable::Message(m) => Event::Message(m.into_event(document_id)),
            EventTable::Postback(p) => Event::Postback(p.into_event(document_id)),
            EventTable::VideoPlayComplete(v) => Event::VideoPlayComplete(v.into_event(document_id)),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FollowEventTable {
    reply_token: String,
    webhook_event_id: String,
    delivery_context: DeliveryContextTable,
    mode: String,
    communication_type: CommunicationTypeTable,
    sending_type: SendingTypeTable,
    sending_method: SendingMethodTable,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

impl FollowEventTable {
    pub fn into_event(&self, document_id: String) -> FollowEvent {
        FollowEvent {
            id: Id::try_from(document_id).unwrap(),
            reply_token: self.reply_token.clone(),
            delivery_context: DeliveryContext::from(self.delivery_context.clone()),
            mode: self.mode.clone(),
            webhook_event_id: self.webhook_event_id.clone(),
            created_at: self.created_at,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UnfollowEventTable {
    webhook_event_id: String,
    delivery_context: DeliveryContextTable,
    mode: String,
    communication_type: CommunicationTypeTable,
    sending_type: SendingTypeTable,
    sending_method: SendingMethodTable,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

impl UnfollowEventTable {
    fn into_event(&self, document_id: String) -> UnfollowEvent {
        UnfollowEvent {
            id: Id::try_from(document_id).unwrap(),
            delivery_context: DeliveryContext::from(self.delivery_context.clone()),
            mode: self.mode.clone(),
            webhook_event_id: self.webhook_event_id.clone(),
            created_at: self.created_at,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MessageEventTable {
    reply_token: String,
    webhook_event_id: String,
    delivery_context: DeliveryContextTable,
    mode: String,
    communication_type: CommunicationTypeTable,
    sending_type: SendingTypeTable,
    sending_method: SendingMethodTable,
    pub message: MessageTable,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

impl MessageEventTable {
    fn into_event(&self, document_id: String) -> MessageEvent {
        MessageEvent {
            id: Id::try_from(document_id).unwrap(),
            reply_token: self.reply_token.clone(),
            delivery_context: DeliveryContext::from(self.delivery_context.clone()),
            mode: self.mode.clone(),
            webhook_event_id: self.webhook_event_id.clone(),
            message: self.message.clone().into(),
            created_at: self.created_at,
        }
    }
}

impl From<MessageTable> for Message {
    fn from(m: MessageTable) -> Self {
        match m {
            MessageTable::Text(t) => Message::Text(t.into()),
            MessageTable::Image(i) => Message::Image(i.into()),
            MessageTable::Video(v) => Message::Video(v.into()),
            MessageTable::Audio(a) => Message::Audio(a.into()),
            MessageTable::File(f) => Message::File(f.into()),
            MessageTable::Location(l) => Message::Location(l.into()),
            MessageTable::Sticker(s) => Message::Sticker(s.into()),
        }
    }
}

impl From<TextMessageTable> for TextMessage {
    fn from(m: TextMessageTable) -> Self {
        Self {
            id: m.id,
            text: m.text,
            emojis: m.emojis.into_iter().map(|e| e.into()).collect(),
        }
    }
}

impl From<EmojiTable> for Emoji {
    fn from(e: EmojiTable) -> Self {
        Self {
            index: e.index,
            length: e.length,
            product_id: e.product_id,
            emoji_id: e.emoji_id,
        }
    }
}

impl From<ImageMessageTable> for ImageMessage {
    fn from(m: ImageMessageTable) -> Self {
        Self {
            id: m.id,
            content_provider: m.content_provider.into(),
            image_set: m.image_set.map(|i| i.into()),
        }
    }
}

impl From<ImageSetTable> for ImageSet {
    fn from(i: ImageSetTable) -> Self {
        Self {
            id: i.id,
            index: i.index,
            length: i.length,
        }
    }
}

impl From<VideoMessageTable> for VideoMessage {
    fn from(m: VideoMessageTable) -> Self {
        Self {
            id: m.id,
            duration: m.duration,
            content_provider: m.content_provider.into(),
        }
    }
}

impl From<AudioMessageTable> for AudioMessage {
    fn from(m: AudioMessageTable) -> Self {
        Self {
            id: m.id,
            duration: m.duration,
            content_provider: m.content_provider.into(),
        }
    }
}

impl From<FileMessageTable> for FileMessage {
    fn from(m: FileMessageTable) -> Self {
        Self {
            id: m.id,
            file_name: m.file_name,
            file_size: m.file_size,
        }
    }
}

impl From<LocationMessageTable> for LocationMessage {
    fn from(m: LocationMessageTable) -> Self {
        Self {
            id: m.id,
            title: m.title,
            address: m.address,
            latitude: Decimal::from_f64(m.latitude)
                .unwrap_or_else(|| panic!("Failed to convert f64 {} to Decimal", m.latitude)),
            longitude: Decimal::from_f64(m.latitude)
                .unwrap_or_else(|| panic!("Failed to convert f64 {} to Decimal", m.latitude)),
        }
    }
}

impl From<StickerMessageTable> for StickerMessage {
    fn from(m: StickerMessageTable) -> Self {
        Self {
            id: m.id,
            package_id: m.package_id,
            sticker_id: m.sticker_id,
            sticker_resource_type: m.sticker_resource_type.into(),
            keywords: m.keywords,
            text: m.text,
        }
    }
}

impl From<StickerResourceTypeTable> for StickerResourceType {
    fn from(s: StickerResourceTypeTable) -> Self {
        match s {
            StickerResourceTypeTable::Static => StickerResourceType::Static,
            StickerResourceTypeTable::Animation => StickerResourceType::Animation,
            StickerResourceTypeTable::Sound => StickerResourceType::Sound,
            StickerResourceTypeTable::AnimationSound => StickerResourceType::AnimationSound,
            StickerResourceTypeTable::Popup => StickerResourceType::Popup,
            StickerResourceTypeTable::PopupSound => StickerResourceType::PopupSound,
            StickerResourceTypeTable::Custom => StickerResourceType::Custom,
            StickerResourceTypeTable::Message => StickerResourceType::Message,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostbackEventTable {
    reply_token: String,
    webhook_event_id: String,
    delivery_context: DeliveryContextTable,
    mode: String,
    communication_type: CommunicationTypeTable,
    sending_type: SendingTypeTable,
    sending_method: SendingMethodTable,
    postback: PostbackTable,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryContextTable {
    is_redelivery: bool,
}

impl From<DeliveryContextTable> for DeliveryContext {
    fn from(d: DeliveryContextTable) -> Self {
        Self {
            is_redelivery: d.is_redelivery,
        }
    }
}

impl PostbackEventTable {
    pub fn into_event(&self, document_id: String) -> PostbackEvent {
        PostbackEvent {
            id: Id::try_from(document_id).unwrap(),
            reply_token: self.reply_token.clone(),
            delivery_context: DeliveryContext::from(self.delivery_context.clone()),
            mode: self.mode.clone(),
            webhook_event_id: self.webhook_event_id.clone(),
            postback: self.postback.clone().into(),
            created_at: self.created_at,
        }
    }
}

impl From<PostbackTable> for Postback {
    fn from(p: PostbackTable) -> Self {
        Self {
            data: p.data,
            params: match p.params {
                PostbackParamsTable::Datetime(p) => PostbackParams::Datetime(p.into()),
                PostbackParamsTable::RichMenu(p) => PostbackParams::RichMenu(p.into()),
            },
        }
    }
}

impl From<PostbackDatetimeParamsTable> for PostbackDatetimeParams {
    fn from(p: PostbackDatetimeParamsTable) -> Self {
        match p {
            PostbackDatetimeParamsTable::DateTime(d) => PostbackDatetimeParams::DateTime(d),
            PostbackDatetimeParamsTable::Date(d) => PostbackDatetimeParams::Date(d),
            PostbackDatetimeParamsTable::Time(t) => PostbackDatetimeParams::Time(t),
        }
    }
}

impl From<PostbackRichMenuParamsTable> for PostbackRichMenuParams {
    fn from(p: PostbackRichMenuParamsTable) -> Self {
        Self {
            new_rich_menu_alias_id: p.new_rich_menu_alias_id,
            status: p.status,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VideoPlayCompleteEventTable {
    reply_token: String,
    webhook_event_id: String,
    delivery_context: DeliveryContextTable,
    mode: String,
    communication_type: CommunicationTypeTable,
    sending_type: SendingTypeTable,
    sending_method: SendingMethodTable,
    video_play_complete: VideoPlayCompleteTable,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

impl VideoPlayCompleteEventTable {
    pub fn into_event(&self, document_id: String) -> VideoPlayCompleteEvent {
        VideoPlayCompleteEvent {
            id: Id::try_from(document_id).unwrap(),
            reply_token: self.reply_token.clone(),
            delivery_context: DeliveryContext::from(self.delivery_context.clone()),
            mode: self.mode.clone(),
            webhook_event_id: self.webhook_event_id.clone(),
            video_play_complete: self.video_play_complete.clone().into(),
            created_at: self.created_at,
        }
    }
}

impl From<VideoPlayCompleteTable> for VideoPlayComplete {
    fn from(v: VideoPlayCompleteTable) -> Self {
        Self {
            tracking_id: v.tracking_id,
        }
    }
}

#[derive(Serialize, Deserialize, Display, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CommunicationTypeTable {
    Send,
    Receive,
}

#[derive(Serialize, Deserialize, Display, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SendingTypeTable {
    Manual,
    Bot,
}

#[derive(Serialize, Deserialize, Display, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SendingMethodTable {
    Reply,
    Push,
}

// todo 以下は未使用。確認
#[derive(Serialize, Deserialize, Clone)]
pub struct SenderTable {
    id: i64,
    name: String,
    picture_url: String,
    email: String,
    sender_role: SenderRoleTable,
}

#[derive(Serialize, Deserialize, Display, Clone, Debug)]
#[serde(rename_all = "lowercase")]
enum SenderRoleTable {
    Sender,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostbackTable {
    pub data: String,
    pub params: PostbackParamsTable,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PostbackParamsTable {
    Datetime(PostbackDatetimeParamsTable),
    RichMenu(PostbackRichMenuParamsTable),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum PostbackDatetimeParamsTable {
    #[serde(rename(serialize = "datetime"))]
    DateTime(String),
    #[serde(rename(serialize = "date"))]
    Date(String),
    #[serde(rename(serialize = "time"))]
    Time(String),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostbackRichMenuParamsTable {
    pub new_rich_menu_alias_id: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VideoPlayCompleteTable {
    pub tracking_id: String,
}

#[derive(Serialize, Deserialize, Display, Clone, Debug)]
#[serde(tag = "messageType")] // JSONにmessageTypeというフィールドでタグ名を含む
#[serde(rename_all = "lowercase")]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TextMessageTable {
    pub id: String,
    pub text: String,
    pub emojis: Vec<EmojiTable>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EmojiTable {
    pub index: i32,
    pub length: i32,
    pub product_id: String,
    pub emoji_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ImageMessageTable {
    pub id: String,
    pub content_provider: ContentProviderTable,
    pub image_set: Option<ImageSetTable>,
}

#[derive(Serialize, Deserialize, Display, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum ContentProviderTable {
    Line,
    External(ExternalContentProviderTable),
}

impl From<ContentProviderTable> for ContentProvider {
    fn from(s: ContentProviderTable) -> Self {
        match s {
            ContentProviderTable::Line => ContentProvider::Line,
            ContentProviderTable::External(t) => ContentProvider::External(t.into()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExternalContentProviderTable {
    original_content_url: String,
    preview_image_url: Option<String>,
}

impl From<ExternalContentProviderTable> for ExternalContentProvider {
    fn from(t: ExternalContentProviderTable) -> Self {
        ExternalContentProvider::new(t.original_content_url, t.preview_image_url)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ImageSetTable {
    pub id: String,
    pub index: i32,
    pub length: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VideoMessageTable {
    pub id: String,
    pub duration: i32,
    pub content_provider: ContentProviderTable,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AudioMessageTable {
    pub id: String,
    pub duration: i32,
    pub content_provider: ContentProviderTable,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileMessageTable {
    pub id: String,
    pub file_name: String,
    pub file_size: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LocationMessageTable {
    pub id: String,
    pub title: String,
    pub address: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StickerMessageTable {
    pub id: String,
    pub package_id: String,
    pub sticker_id: String,
    pub sticker_resource_type: StickerResourceTypeTable,
    pub keywords: Option<Vec<String>>,
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Display, Clone, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum StickerResourceTypeTable {
    Static,
    Animation,
    Sound,
    AnimationSound,
    Popup,
    PopupSound,
    Custom,
    Message,
}

impl From<NewEvent> for EventTable {
    fn from(e: NewEvent) -> Self {
        match e {
            NewEvent::Follow(f) => EventTable::Follow(f.into()),
            NewEvent::Unfollow(u) => EventTable::Unfollow(u.into()),
            NewEvent::Message(m) => EventTable::Message(m.into()),
            NewEvent::Postback(p) => EventTable::Postback(p.into()),
            NewEvent::VideoPlayComplete(v) => EventTable::VideoPlayComplete(v.into()),
        }
    }
}

impl From<NewFollowEvent> for FollowEventTable {
    fn from(e: NewFollowEvent) -> Self {
        FollowEventTable {
            reply_token: e.reply_token,
            webhook_event_id: e.webhook_event_id,
            delivery_context: e.delivery_context.into(),
            mode: e.mode,
            communication_type: CommunicationTypeTable::Receive,
            sending_type: SendingTypeTable::Bot,
            sending_method: SendingMethodTable::Reply,
            created_at: e.created_at,
            updated_at: e.created_at,
        }
    }
}

impl From<NewUnfollowEvent> for UnfollowEventTable {
    fn from(e: NewUnfollowEvent) -> Self {
        UnfollowEventTable {
            webhook_event_id: e.webhook_event_id,
            delivery_context: e.delivery_context.into(),
            mode: e.mode,
            communication_type: CommunicationTypeTable::Receive,
            sending_type: SendingTypeTable::Bot,
            sending_method: SendingMethodTable::Reply,
            created_at: e.created_at,
            updated_at: e.created_at,
        }
    }
}

impl From<NewMessageEvent> for MessageEventTable {
    fn from(e: NewMessageEvent) -> Self {
        MessageEventTable {
            reply_token: e.reply_token,
            webhook_event_id: e.webhook_event_id,
            delivery_context: e.delivery_context.into(),
            mode: e.mode,
            communication_type: CommunicationTypeTable::Receive,
            sending_type: SendingTypeTable::Bot,
            sending_method: SendingMethodTable::Reply,
            message: e.message.into(),
            created_at: e.created_at,
            updated_at: e.created_at,
        }
    }
}

impl From<NewPostbackEvent> for PostbackEventTable {
    fn from(e: NewPostbackEvent) -> Self {
        PostbackEventTable {
            reply_token: e.reply_token,
            webhook_event_id: e.webhook_event_id,
            delivery_context: e.delivery_context.into(),
            mode: e.mode,
            communication_type: CommunicationTypeTable::Receive,
            sending_type: SendingTypeTable::Bot,
            sending_method: SendingMethodTable::Reply,
            postback: PostbackTable {
                data: e.postback.data,
                params: match e.postback.params {
                    NewPostbackParams::Datetime(p) => PostbackParamsTable::Datetime(p.into()),
                    NewPostbackParams::RichMenu(p) => PostbackParamsTable::RichMenu(p.into()),
                },
            },
            created_at: e.created_at,
            updated_at: e.created_at,
        }
    }
}

impl From<NewDeliveryContext> for DeliveryContextTable {
    fn from(d: NewDeliveryContext) -> Self {
        DeliveryContextTable {
            is_redelivery: d.is_redelivery,
        }
    }
}

impl From<NewPostbackDatetimeParams> for PostbackDatetimeParamsTable {
    fn from(p: NewPostbackDatetimeParams) -> Self {
        match p {
            NewPostbackDatetimeParams::DateTime(p) => PostbackDatetimeParamsTable::DateTime(p),
            NewPostbackDatetimeParams::Date(p) => PostbackDatetimeParamsTable::Date(p),
            NewPostbackDatetimeParams::Time(p) => PostbackDatetimeParamsTable::Time(p),
        }
    }
}

impl From<NewPostbackRichMenuParams> for PostbackRichMenuParamsTable {
    fn from(p: NewPostbackRichMenuParams) -> Self {
        PostbackRichMenuParamsTable {
            new_rich_menu_alias_id: p.new_rich_menu_alias_id,
            status: p.status,
        }
    }
}

impl From<NewVideoPlayCompleteEvent> for VideoPlayCompleteEventTable {
    fn from(e: NewVideoPlayCompleteEvent) -> Self {
        VideoPlayCompleteEventTable {
            reply_token: e.reply_token,
            webhook_event_id: e.webhook_event_id,
            delivery_context: DeliveryContextTable {
                is_redelivery: e.delivery_context.is_redelivery,
            },
            mode: e.mode,
            communication_type: CommunicationTypeTable::Receive,
            sending_type: SendingTypeTable::Bot,
            sending_method: SendingMethodTable::Reply,
            video_play_complete: VideoPlayCompleteTable {
                tracking_id: e.video_play_complete.tracking_id,
            },
            created_at: e.created_at,
            updated_at: e.created_at,
        }
    }
}

// from NewMessage to MessageTable
impl From<NewMessage> for MessageTable {
    fn from(m: NewMessage) -> Self {
        match m {
            NewMessage::Text(t) => MessageTable::Text(t.into()),
            NewMessage::Image(i) => MessageTable::Image(i.into()),
            NewMessage::Video(v) => MessageTable::Video(v.into()),
            NewMessage::Audio(a) => MessageTable::Audio(a.into()),
            NewMessage::File(f) => MessageTable::File(f.into()),
            NewMessage::Location(l) => MessageTable::Location(l.into()),
            NewMessage::Sticker(s) => MessageTable::Sticker(s.into()),
        }
    }
}

impl From<NewTextMessage> for TextMessageTable {
    fn from(t: NewTextMessage) -> Self {
        TextMessageTable {
            id: t.id,
            text: t.text,
            emojis: t.emojis.into_iter().map(|e| e.into()).collect(),
        }
    }
}

impl From<NewEmoji> for EmojiTable {
    fn from(e: NewEmoji) -> Self {
        EmojiTable {
            index: e.index,
            length: e.length,
            product_id: e.product_id,
            emoji_id: e.emoji_id,
        }
    }
}

impl From<NewImageMessage> for ImageMessageTable {
    fn from(i: NewImageMessage) -> Self {
        ImageMessageTable {
            id: i.id,
            content_provider: i.content_provider.into(),
            image_set: i.image_set.map(|i| i.into()),
        }
    }
}

impl From<NewContentProvider> for ContentProviderTable {
    fn from(s: NewContentProvider) -> Self {
        match s {
            NewContentProvider::Line => ContentProviderTable::Line,
            NewContentProvider::External(e) => ContentProviderTable::External(e.into()),
        }
    }
}

impl From<NewExternalContentProvider> for ExternalContentProviderTable {
    fn from(s: NewExternalContentProvider) -> Self {
        ExternalContentProviderTable {
            original_content_url: s.original_content_url,
            preview_image_url: s.preview_image_url,
        }
    }
}

impl From<NewImageSet> for ImageSetTable {
    fn from(i: NewImageSet) -> Self {
        ImageSetTable {
            id: i.id,
            index: i.index,
            length: i.length,
        }
    }
}

impl From<NewVideoMessage> for VideoMessageTable {
    fn from(v: NewVideoMessage) -> Self {
        VideoMessageTable {
            id: v.id,
            duration: v.duration,
            content_provider: v.content_provider.into(),
        }
    }
}

impl From<NewAudioMessage> for AudioMessageTable {
    fn from(a: NewAudioMessage) -> Self {
        AudioMessageTable {
            id: a.id,
            duration: a.duration,
            content_provider: a.content_provider.into(),
        }
    }
}

impl From<NewFileMessage> for FileMessageTable {
    fn from(f: NewFileMessage) -> Self {
        FileMessageTable {
            id: f.id,
            file_name: f.file_name,
            file_size: f.file_size,
        }
    }
}

impl From<NewLocationMessage> for LocationMessageTable {
    fn from(l: NewLocationMessage) -> Self {
        LocationMessageTable {
            id: l.id,
            title: l.title,
            address: l.address,
            latitude: l
                .latitude
                .to_f64()
                .unwrap_or_else(|| panic!("Failed to convert Decimal {} to f64", l.latitude)),
            longitude: l
                .longitude
                .to_f64()
                .unwrap_or_else(|| panic!("Failed to convert Decimal {} to f64", l.longitude)),
        }
    }
}

impl From<NewStickerMessage> for StickerMessageTable {
    fn from(s: NewStickerMessage) -> Self {
        StickerMessageTable {
            id: s.id,
            package_id: s.package_id,
            sticker_id: s.sticker_id,
            sticker_resource_type: s.sticker_resource_type.into(),
            keywords: s.keywords,
            text: s.text,
        }
    }
}

impl From<NewStickerResourceType> for StickerResourceTypeTable {
    fn from(s: NewStickerResourceType) -> Self {
        match s {
            NewStickerResourceType::Static => StickerResourceTypeTable::Static,
            NewStickerResourceType::Animation => StickerResourceTypeTable::Animation,
            NewStickerResourceType::Sound => StickerResourceTypeTable::Sound,
            NewStickerResourceType::AnimationSound => StickerResourceTypeTable::AnimationSound,
            NewStickerResourceType::Popup => StickerResourceTypeTable::Popup,
            NewStickerResourceType::PopupSound => StickerResourceTypeTable::PopupSound,
            NewStickerResourceType::Custom => StickerResourceTypeTable::Custom,
            NewStickerResourceType::Message => StickerResourceTypeTable::Message,
        }
    }
}
