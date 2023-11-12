use chrono::{DateTime, Local};
use rust_decimal::{
    prelude::{FromPrimitive, ToPrimitive},
    Decimal,
};
use serde::{Deserialize, Serialize};
use strum_macros::Display;

use domain::model::{
    message::event::{
        Event, EventContentProvider, EventContentProviderExternal, EventDeliveryContext,
        EventEmoji, EventFollow, EventImageSet, EventMessage, EventMessageContent,
        EventMessageContentAudio, EventMessageContentFile, EventMessageContentImage,
        EventMessageContentLocation, EventMessageContentSticker, EventMessageContentText,
        EventMessageContentVideo, EventPostback, EventPostbackContent, EventPostbackParams,
        EventPostbackParamsDatetime, EventPostbackParamsRichMenu, EventStickerResourceType,
        EventUnfollow, EventVideoPlayComplete, EventVideoPlayCompleteContent, NewEvent,
        NewEventContentProvider, NewEventContentProviderExternal, NewEventDeliveryContext,
        NewEventEmoji, NewEventFollow, NewEventImageSet, NewEventMessage, NewEventMessageContent,
        NewEventMessageContentAudio, NewEventMessageContentFile, NewEventMessageContentImage,
        NewEventMessageContentLocation, NewEventMessageContentSticker, NewEventMessageContentText,
        NewEventMessageContentVideo, NewEventPostback, NewEventPostbackParams,
        NewEventPostbackParamsDatetime, NewEventPostbackParamsRichMenu,
        NewEventStickerResourceType, NewEventUnfollow, NewEventVideoPlayComplete,
    },
    Id,
};

#[derive(Serialize, Deserialize, Display, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum EventTable {
    Follow(EventFollowTable),
    Unfollow(EventUnfollowTable),
    Postback(EventPostbackTable),
    VideoPlayComplete(EventVideoPlayCompleteTable),
    Message(EventMessageTable),
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
    pub fn into_event(&self, document_id: &String) -> Event {
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
pub struct EventFollowTable {
    reply_token: String,
    webhook_event_id: String,
    delivery_context: EventDeliveryContextTable,
    mode: String,
    communication_type: EventCommunicationTypeTable,
    sending_type: EventSendingTypeTable,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

impl EventFollowTable {
    pub fn into_event(&self, document_id: &String) -> EventFollow {
        EventFollow {
            id: Id::try_from(document_id.to_string())
                .unwrap_or_else(|_| panic!("Failed to convert String {} to UUID", document_id)),
            reply_token: self.reply_token.clone(),
            delivery_context: EventDeliveryContext::from(self.delivery_context.clone()),
            mode: self.mode.clone(),
            webhook_event_id: self.webhook_event_id.clone(),
            created_at: self.created_at,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventUnfollowTable {
    webhook_event_id: String,
    delivery_context: EventDeliveryContextTable,
    mode: String,
    communication_type: EventCommunicationTypeTable,
    sending_type: EventSendingTypeTable,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

impl EventUnfollowTable {
    fn into_event(&self, document_id: &String) -> EventUnfollow {
        EventUnfollow {
            id: Id::try_from(document_id.to_string())
                .unwrap_or_else(|_| panic!("Failed to convert String {} to UUID", document_id)),
            delivery_context: EventDeliveryContext::from(self.delivery_context.clone()),
            mode: self.mode.clone(),
            webhook_event_id: self.webhook_event_id.clone(),
            created_at: self.created_at,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventMessageTable {
    reply_token: String,
    webhook_event_id: String,
    delivery_context: EventDeliveryContextTable,
    mode: String,
    communication_type: EventCommunicationTypeTable,
    sending_type: EventSendingTypeTable,
    pub messages: Vec<EventMessageContentTable>,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

impl EventMessageTable {
    fn into_event(&self, document_id: &String) -> EventMessage {
        EventMessage {
            id: Id::try_from(document_id.to_string())
                .unwrap_or_else(|_| panic!("Failed to convert String {} to UUID", document_id)),
            reply_token: self.reply_token.clone(),
            delivery_context: EventDeliveryContext::from(self.delivery_context.clone()),
            mode: self.mode.clone(),
            webhook_event_id: self.webhook_event_id.clone(),
            message: self.messages[0].clone().into(),
            created_at: self.created_at,
        }
    }
}

impl From<EventMessageContentTable> for EventMessageContent {
    fn from(m: EventMessageContentTable) -> Self {
        match m {
            EventMessageContentTable::Text(t) => EventMessageContent::Text(t.into()),
            EventMessageContentTable::Image(i) => EventMessageContent::Image(i.into()),
            EventMessageContentTable::Video(v) => EventMessageContent::Video(v.into()),
            EventMessageContentTable::Audio(a) => EventMessageContent::Audio(a.into()),
            EventMessageContentTable::File(f) => EventMessageContent::File(f.into()),
            EventMessageContentTable::Location(l) => EventMessageContent::Location(l.into()),
            EventMessageContentTable::Sticker(s) => EventMessageContent::Sticker(s.into()),
        }
    }
}

impl From<EventMessageContentTextTable> for EventMessageContentText {
    fn from(m: EventMessageContentTextTable) -> Self {
        Self {
            id: m.id,
            text: m.text,
            emojis: m.emojis.into_iter().map(|e| e.into()).collect(),
        }
    }
}

impl From<EventEmojiTable> for EventEmoji {
    fn from(e: EventEmojiTable) -> Self {
        Self {
            index: e.index,
            length: e.length,
            product_id: e.product_id,
            emoji_id: e.emoji_id,
        }
    }
}

impl From<EventMessageContentImageTable> for EventMessageContentImage {
    fn from(m: EventMessageContentImageTable) -> Self {
        Self {
            id: m.id,
            content_provider: m.content_provider.into(),
            image_set: m.image_set.map(|i| i.into()),
        }
    }
}

impl From<EventImageSetTable> for EventImageSet {
    fn from(i: EventImageSetTable) -> Self {
        Self {
            id: i.id,
            index: i.index,
            length: i.length,
        }
    }
}

impl From<EventMessageContentVideoTable> for EventMessageContentVideo {
    fn from(m: EventMessageContentVideoTable) -> Self {
        Self {
            id: m.id,
            duration: m.duration,
            content_provider: m.content_provider.into(),
        }
    }
}

impl From<EventMessageContentAudioTable> for EventMessageContentAudio {
    fn from(m: EventMessageContentAudioTable) -> Self {
        Self {
            id: m.id,
            duration: m.duration,
            content_provider: m.content_provider.into(),
        }
    }
}

impl From<EventMessageContentFileTable> for EventMessageContentFile {
    fn from(m: EventMessageContentFileTable) -> Self {
        Self {
            id: m.id,
            file_name: m.file_name,
            file_size: m.file_size,
        }
    }
}

impl From<EventMessageContentLocationTable> for EventMessageContentLocation {
    fn from(m: EventMessageContentLocationTable) -> Self {
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

impl From<EventMessageContentStickerTable> for EventMessageContentSticker {
    fn from(m: EventMessageContentStickerTable) -> Self {
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

impl From<EventStickerResourceTypeTable> for EventStickerResourceType {
    fn from(s: EventStickerResourceTypeTable) -> Self {
        match s {
            EventStickerResourceTypeTable::Static => EventStickerResourceType::Static,
            EventStickerResourceTypeTable::Animation => EventStickerResourceType::Animation,
            EventStickerResourceTypeTable::Sound => EventStickerResourceType::Sound,
            EventStickerResourceTypeTable::AnimationSound => {
                EventStickerResourceType::AnimationSound
            }
            EventStickerResourceTypeTable::Popup => EventStickerResourceType::Popup,
            EventStickerResourceTypeTable::PopupSound => EventStickerResourceType::PopupSound,
            EventStickerResourceTypeTable::Custom => EventStickerResourceType::Custom,
            EventStickerResourceTypeTable::Message => EventStickerResourceType::Message,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventPostbackTable {
    reply_token: String,
    webhook_event_id: String,
    delivery_context: EventDeliveryContextTable,
    mode: String,
    communication_type: EventCommunicationTypeTable,
    sending_type: EventSendingTypeTable,
    postback: EventPostbackContentTable,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventDeliveryContextTable {
    is_redelivery: bool,
}

impl From<EventDeliveryContextTable> for EventDeliveryContext {
    fn from(d: EventDeliveryContextTable) -> Self {
        Self {
            is_redelivery: d.is_redelivery,
        }
    }
}

impl EventPostbackTable {
    pub fn into_event(&self, document_id: &String) -> EventPostback {
        EventPostback {
            id: Id::try_from(document_id.to_string())
                .unwrap_or_else(|_| panic!("Failed to convert String {} to UUID", document_id)),
            reply_token: self.reply_token.clone(),
            delivery_context: EventDeliveryContext::from(self.delivery_context.clone()),
            mode: self.mode.clone(),
            webhook_event_id: self.webhook_event_id.clone(),
            postback: self.postback.clone().into(),
            created_at: self.created_at,
        }
    }
}

impl From<EventPostbackContentTable> for EventPostbackContent {
    fn from(p: EventPostbackContentTable) -> Self {
        Self {
            data: p.data,
            params: match p.params {
                EventPostbackParamsTable::Datetime(p) => EventPostbackParams::Datetime(p.into()),
                EventPostbackParamsTable::RichMenu(p) => EventPostbackParams::RichMenu(p.into()),
            },
        }
    }
}

impl From<EventPostbackParamsDatetimeTable> for EventPostbackParamsDatetime {
    fn from(p: EventPostbackParamsDatetimeTable) -> Self {
        match p {
            EventPostbackParamsDatetimeTable::DateTime(d) => {
                EventPostbackParamsDatetime::DateTime(d)
            }
            EventPostbackParamsDatetimeTable::Date(d) => EventPostbackParamsDatetime::Date(d),
            EventPostbackParamsDatetimeTable::Time(t) => EventPostbackParamsDatetime::Time(t),
        }
    }
}

impl From<EventPostbackParamsRichMenuTable> for EventPostbackParamsRichMenu {
    fn from(p: EventPostbackParamsRichMenuTable) -> Self {
        Self {
            new_rich_menu_alias_id: p.new_rich_menu_alias_id,
            status: p.status,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventVideoPlayCompleteTable {
    reply_token: String,
    webhook_event_id: String,
    delivery_context: EventDeliveryContextTable,
    mode: String,
    communication_type: EventCommunicationTypeTable,
    sending_type: EventSendingTypeTable,
    video_play_complete: EventVideoPlayCompleteContentTable,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

impl EventVideoPlayCompleteTable {
    pub fn into_event(&self, document_id: &String) -> EventVideoPlayComplete {
        EventVideoPlayComplete {
            id: Id::try_from(document_id.to_string())
                .unwrap_or_else(|_| panic!("Failed to convert String {} to UUID", document_id)),
            reply_token: self.reply_token.clone(),
            delivery_context: EventDeliveryContext::from(self.delivery_context.clone()),
            mode: self.mode.clone(),
            webhook_event_id: self.webhook_event_id.clone(),
            video_play_complete: self.video_play_complete.clone().into(),
            created_at: self.created_at,
        }
    }
}

impl From<EventVideoPlayCompleteContentTable> for EventVideoPlayCompleteContent {
    fn from(v: EventVideoPlayCompleteContentTable) -> Self {
        Self {
            tracking_id: v.tracking_id,
        }
    }
}

#[derive(Serialize, Deserialize, Display, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum EventCommunicationTypeTable {
    Receive,
}

#[derive(Serialize, Deserialize, Display, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum EventSendingTypeTable {
    Manual,
    Bot,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EventPostbackContentTable {
    pub data: String,
    pub params: EventPostbackParamsTable,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum EventPostbackParamsTable {
    Datetime(EventPostbackParamsDatetimeTable),
    RichMenu(EventPostbackParamsRichMenuTable),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum EventPostbackParamsDatetimeTable {
    #[serde(rename(serialize = "datetime"))]
    DateTime(String),
    #[serde(rename(serialize = "date"))]
    Date(String),
    #[serde(rename(serialize = "time"))]
    Time(String),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventPostbackParamsRichMenuTable {
    pub new_rich_menu_alias_id: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventVideoPlayCompleteContentTable {
    pub tracking_id: String,
}

#[derive(Serialize, Deserialize, Display, Clone, Debug)]
#[serde(tag = "messageType")] // JSONにmessageTypeというフィールドでタグ名を含む
#[serde(rename_all = "lowercase")]
pub enum EventMessageContentTable {
    #[strum(serialize = "text")]
    Text(EventMessageContentTextTable),
    #[strum(serialize = "image")]
    Image(EventMessageContentImageTable),
    #[strum(serialize = "video")]
    Video(EventMessageContentVideoTable),
    #[strum(serialize = "audio")]
    Audio(EventMessageContentAudioTable),
    #[strum(serialize = "file")]
    File(EventMessageContentFileTable),
    #[strum(serialize = "location")]
    Location(EventMessageContentLocationTable),
    #[strum(serialize = "sticker")]
    Sticker(EventMessageContentStickerTable),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EventMessageContentTextTable {
    pub id: String,
    pub text: String,
    pub emojis: Vec<EventEmojiTable>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventEmojiTable {
    pub index: i32,
    pub length: i32,
    pub product_id: String,
    pub emoji_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventMessageContentImageTable {
    pub id: String,
    pub content_provider: EventContentProviderTable,
    pub image_set: Option<EventImageSetTable>,
}

#[derive(Serialize, Deserialize, Display, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum EventContentProviderTable {
    Line,
    External(EventContentProviderExternalTable),
}

impl From<EventContentProviderTable> for EventContentProvider {
    fn from(s: EventContentProviderTable) -> Self {
        match s {
            EventContentProviderTable::Line => EventContentProvider::Line,
            EventContentProviderTable::External(t) => EventContentProvider::External(t.into()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventContentProviderExternalTable {
    original_content_url: String,
    preview_image_url: Option<String>,
}

impl From<EventContentProviderExternalTable> for EventContentProviderExternal {
    fn from(t: EventContentProviderExternalTable) -> Self {
        EventContentProviderExternal::new(t.original_content_url, t.preview_image_url)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EventImageSetTable {
    pub id: String,
    pub index: i32,
    pub length: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventMessageContentVideoTable {
    pub id: String,
    pub duration: i32,
    pub content_provider: EventContentProviderTable,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventMessageContentAudioTable {
    pub id: String,
    pub duration: i32,
    pub content_provider: EventContentProviderTable,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventMessageContentFileTable {
    pub id: String,
    pub file_name: String,
    pub file_size: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EventMessageContentLocationTable {
    pub id: String,
    pub title: String,
    pub address: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventMessageContentStickerTable {
    pub id: String,
    pub package_id: String,
    pub sticker_id: String,
    pub sticker_resource_type: EventStickerResourceTypeTable,
    pub keywords: Option<Vec<String>>,
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Display, Clone, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum EventStickerResourceTypeTable {
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

impl From<NewEventFollow> for EventFollowTable {
    fn from(e: NewEventFollow) -> Self {
        EventFollowTable {
            reply_token: e.reply_token,
            webhook_event_id: e.webhook_event_id,
            delivery_context: e.delivery_context.into(),
            mode: e.mode,
            communication_type: EventCommunicationTypeTable::Receive,
            sending_type: EventSendingTypeTable::Bot,
            created_at: e.created_at,
            updated_at: e.created_at,
        }
    }
}

impl From<NewEventUnfollow> for EventUnfollowTable {
    fn from(e: NewEventUnfollow) -> Self {
        EventUnfollowTable {
            webhook_event_id: e.webhook_event_id,
            delivery_context: e.delivery_context.into(),
            mode: e.mode,
            communication_type: EventCommunicationTypeTable::Receive,
            sending_type: EventSendingTypeTable::Bot,
            created_at: e.created_at,
            updated_at: e.created_at,
        }
    }
}

impl From<NewEventMessage> for EventMessageTable {
    fn from(e: NewEventMessage) -> Self {
        EventMessageTable {
            reply_token: e.reply_token,
            webhook_event_id: e.webhook_event_id,
            delivery_context: e.delivery_context.into(),
            mode: e.mode,
            communication_type: EventCommunicationTypeTable::Receive,
            sending_type: EventSendingTypeTable::Bot,
            messages: vec![e.message.into()],
            created_at: e.created_at,
            updated_at: e.created_at,
        }
    }
}

impl From<NewEventPostback> for EventPostbackTable {
    fn from(e: NewEventPostback) -> Self {
        EventPostbackTable {
            reply_token: e.reply_token,
            webhook_event_id: e.webhook_event_id,
            delivery_context: e.delivery_context.into(),
            mode: e.mode,
            communication_type: EventCommunicationTypeTable::Receive,
            sending_type: EventSendingTypeTable::Bot,
            postback: EventPostbackContentTable {
                data: e.postback.data,
                params: match e.postback.params {
                    NewEventPostbackParams::Datetime(p) => {
                        EventPostbackParamsTable::Datetime(p.into())
                    }
                    NewEventPostbackParams::RichMenu(p) => {
                        EventPostbackParamsTable::RichMenu(p.into())
                    }
                },
            },
            created_at: e.created_at,
            updated_at: e.created_at,
        }
    }
}

impl From<NewEventDeliveryContext> for EventDeliveryContextTable {
    fn from(d: NewEventDeliveryContext) -> Self {
        EventDeliveryContextTable {
            is_redelivery: d.is_redelivery,
        }
    }
}

impl From<NewEventPostbackParamsDatetime> for EventPostbackParamsDatetimeTable {
    fn from(p: NewEventPostbackParamsDatetime) -> Self {
        match p {
            NewEventPostbackParamsDatetime::DateTime(p) => {
                EventPostbackParamsDatetimeTable::DateTime(p)
            }
            NewEventPostbackParamsDatetime::Date(p) => EventPostbackParamsDatetimeTable::Date(p),
            NewEventPostbackParamsDatetime::Time(p) => EventPostbackParamsDatetimeTable::Time(p),
        }
    }
}

impl From<NewEventPostbackParamsRichMenu> for EventPostbackParamsRichMenuTable {
    fn from(p: NewEventPostbackParamsRichMenu) -> Self {
        EventPostbackParamsRichMenuTable {
            new_rich_menu_alias_id: p.new_rich_menu_alias_id,
            status: p.status,
        }
    }
}

impl From<NewEventVideoPlayComplete> for EventVideoPlayCompleteTable {
    fn from(e: NewEventVideoPlayComplete) -> Self {
        EventVideoPlayCompleteTable {
            reply_token: e.reply_token,
            webhook_event_id: e.webhook_event_id,
            delivery_context: EventDeliveryContextTable {
                is_redelivery: e.delivery_context.is_redelivery,
            },
            mode: e.mode,
            communication_type: EventCommunicationTypeTable::Receive,
            sending_type: EventSendingTypeTable::Bot,
            video_play_complete: EventVideoPlayCompleteContentTable {
                tracking_id: e.video_play_complete.tracking_id,
            },
            created_at: e.created_at,
            updated_at: e.created_at,
        }
    }
}

impl From<NewEventMessageContent> for EventMessageContentTable {
    fn from(m: NewEventMessageContent) -> Self {
        match m {
            NewEventMessageContent::Text(t) => EventMessageContentTable::Text(t.into()),
            NewEventMessageContent::Image(i) => EventMessageContentTable::Image(i.into()),
            NewEventMessageContent::Video(v) => EventMessageContentTable::Video(v.into()),
            NewEventMessageContent::Audio(a) => EventMessageContentTable::Audio(a.into()),
            NewEventMessageContent::File(f) => EventMessageContentTable::File(f.into()),
            NewEventMessageContent::Location(l) => EventMessageContentTable::Location(l.into()),
            NewEventMessageContent::Sticker(s) => EventMessageContentTable::Sticker(s.into()),
        }
    }
}

impl From<NewEventMessageContentText> for EventMessageContentTextTable {
    fn from(t: NewEventMessageContentText) -> Self {
        EventMessageContentTextTable {
            id: t.id,
            text: t.text,
            emojis: t.emojis.into_iter().map(|e| e.into()).collect(),
        }
    }
}

impl From<NewEventEmoji> for EventEmojiTable {
    fn from(e: NewEventEmoji) -> Self {
        EventEmojiTable {
            index: e.index,
            length: e.length,
            product_id: e.product_id,
            emoji_id: e.emoji_id,
        }
    }
}

impl From<NewEventMessageContentImage> for EventMessageContentImageTable {
    fn from(i: NewEventMessageContentImage) -> Self {
        EventMessageContentImageTable {
            id: i.id,
            content_provider: i.content_provider.into(),
            image_set: i.image_set.map(|i| i.into()),
        }
    }
}

impl From<NewEventContentProvider> for EventContentProviderTable {
    fn from(s: NewEventContentProvider) -> Self {
        match s {
            NewEventContentProvider::Line => EventContentProviderTable::Line,
            NewEventContentProvider::External(e) => EventContentProviderTable::External(e.into()),
        }
    }
}

impl From<NewEventContentProviderExternal> for EventContentProviderExternalTable {
    fn from(s: NewEventContentProviderExternal) -> Self {
        EventContentProviderExternalTable {
            original_content_url: s.original_content_url,
            preview_image_url: s.preview_image_url,
        }
    }
}

impl From<NewEventImageSet> for EventImageSetTable {
    fn from(i: NewEventImageSet) -> Self {
        EventImageSetTable {
            id: i.id,
            index: i.index,
            length: i.length,
        }
    }
}

impl From<NewEventMessageContentVideo> for EventMessageContentVideoTable {
    fn from(v: NewEventMessageContentVideo) -> Self {
        EventMessageContentVideoTable {
            id: v.id,
            duration: v.duration,
            content_provider: v.content_provider.into(),
        }
    }
}

impl From<NewEventMessageContentAudio> for EventMessageContentAudioTable {
    fn from(a: NewEventMessageContentAudio) -> Self {
        EventMessageContentAudioTable {
            id: a.id,
            duration: a.duration,
            content_provider: a.content_provider.into(),
        }
    }
}

impl From<NewEventMessageContentFile> for EventMessageContentFileTable {
    fn from(f: NewEventMessageContentFile) -> Self {
        EventMessageContentFileTable {
            id: f.id,
            file_name: f.file_name,
            file_size: f.file_size,
        }
    }
}

impl From<NewEventMessageContentLocation> for EventMessageContentLocationTable {
    fn from(l: NewEventMessageContentLocation) -> Self {
        EventMessageContentLocationTable {
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

impl From<NewEventMessageContentSticker> for EventMessageContentStickerTable {
    fn from(s: NewEventMessageContentSticker) -> Self {
        EventMessageContentStickerTable {
            id: s.id,
            package_id: s.package_id,
            sticker_id: s.sticker_id,
            sticker_resource_type: s.sticker_resource_type.into(),
            keywords: s.keywords,
            text: s.text,
        }
    }
}

impl From<NewEventStickerResourceType> for EventStickerResourceTypeTable {
    fn from(s: NewEventStickerResourceType) -> Self {
        match s {
            NewEventStickerResourceType::Static => EventStickerResourceTypeTable::Static,
            NewEventStickerResourceType::Animation => EventStickerResourceTypeTable::Animation,
            NewEventStickerResourceType::Sound => EventStickerResourceTypeTable::Sound,
            NewEventStickerResourceType::AnimationSound => {
                EventStickerResourceTypeTable::AnimationSound
            }
            NewEventStickerResourceType::Popup => EventStickerResourceTypeTable::Popup,
            NewEventStickerResourceType::PopupSound => EventStickerResourceTypeTable::PopupSound,
            NewEventStickerResourceType::Custom => EventStickerResourceTypeTable::Custom,
            NewEventStickerResourceType::Message => EventStickerResourceTypeTable::Message,
        }
    }
}
