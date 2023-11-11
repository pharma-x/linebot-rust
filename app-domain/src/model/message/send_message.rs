use chrono::{DateTime, Local};
use rust_decimal::Decimal;

use crate::model::Id;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SendMessages {
    pub id: Id<SendMessage>,
    pub sending_type: SendSendingType,
    pub sending_method: SendSendingMethod,
    pub sender: Option<SendSender>,
    pub messages: Vec<SendMessage>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SendSendingType {
    Bot,
    Manual,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SendSendingMethod {
    Reply,
    Push,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SendSender {
    pub id: i64,
    pub name: String,
    pub picture_url: String,
    pub email: String,
    pub sender_role: SendSenderRole,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SendSenderRole {
    Sender,
}

// TODO Flex Messageの実装
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SendMessage {
    Text(SendMessageText),
    Sticker(SendStickerMessage),
    Image(SendImageMessage),
    Video(SendVideoMessage),
    Audio(SendAudioMessage),
    Location(SendLocationMessage),
    Imagemap(SendImagemapMessage),
    Template(SendTemplateMessage),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SendMessageText {
    pub message_id: String,
    pub text: String,
    pub emojis: Option<Vec<SendEmoji>>,
    pub quote_token: Option<SendQuoteToken>,
    pub created_at: DateTime<Local>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SendEmoji {
    pub index: u32,
    pub product_id: String,
    pub emoji_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SendQuoteToken(pub String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SendStickerMessage {
    pub message_id: String,
    pub package_id: String,
    pub sticker_id: String,
    pub quote_token: Option<SendQuoteToken>,
    pub created_at: DateTime<Local>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SendImageMessage {
    pub message_id: String,
    pub original_content_url: String,
    pub preview_image_url: String,
    pub created_at: DateTime<Local>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SendVideoMessage {
    pub message_id: String,
    pub original_content_url: String,
    pub preview_image_url: String,
    pub tracking_id: Option<String>,
    pub created_at: DateTime<Local>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SendAudioMessage {
    pub message_id: String,
    pub original_content_url: String,
    pub duration: u32,
    pub created_at: DateTime<Local>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SendLocationMessage {
    pub message_id: String,
    pub title: String,
    pub address: String,
    pub latitude: Decimal,
    pub longitude: Decimal,
    pub created_at: DateTime<Local>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SendImagemapMessage {
    pub message_id: String,
    pub base_url: String,
    pub alt_text: String,
    pub base_size: SendImagemapBaseSize,
    pub video: Option<SendImagemapVideo>,
    pub actions: Vec<SendImagemapAction>,
    pub created_at: DateTime<Local>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SendImagemapBaseSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SendImagemapVideo {
    pub original_content_url: String,
    pub preview_image_url: String,
    pub area: SendImagemapVideoArea,
    pub external_link: SendImagemapVideoExternalLink,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SendImagemapVideoArea {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SendImagemapVideoExternalLink {
    pub link_uri: String,
    pub label: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SendImagemapAction {
    Uri(SendImagemapUriAction),
    Message(SendImagemapMessageAction),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SendImagemapUriAction {
    pub label: String,
    pub link_uri: String,
    pub area: SendImagemapActionArea,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SendImagemapMessageAction {
    pub label: String,
    pub text: String,
    pub area: SendImagemapActionArea,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SendImagemapActionArea {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SendTemplateMessage {
    pub message_id: String,
    pub alt_text: String,
    pub template: SendTemplateMessageContent,
    pub created_at: DateTime<Local>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SendTemplateMessageContent {
    Buttons(SendButtonsTemplate),
    Confirm(SendConfirmTemplate),
    Carousel(SendCarouselTemplate),
    ImageCarousel(SendImageCarouselTemplate),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SendButtonsTemplate {
    pub thumbnail_image_url: Option<String>,
    pub image_aspect_ratio: Option<SendImageAspectRatio>,
    pub image_size: Option<SendImageSize>,
    pub image_background_color: Option<String>,
    pub title: Option<String>,
    pub text: String,
    pub default_action: Option<SendTemplateAction>,
    pub actions: Vec<SendTemplateAction>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SendImageAspectRatio {
    Rectangle,
    Square,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SendImageSize {
    Cover,
    Contain,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SendConfirmTemplate {
    pub text: String,
    pub actions: Vec<SendTemplateAction>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SendCarouselTemplate {
    pub columns: Vec<SendCarouselColumn>,
    pub image_aspect_ratio: Option<SendImageAspectRatio>,
    pub image_size: Option<SendImageSize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SendCarouselColumn {
    pub thumbnail_image_url: Option<String>,
    pub image_background_color: Option<String>,
    pub title: Option<String>,
    pub text: String,
    pub default_action: Option<SendTemplateAction>,
    pub actions: Vec<SendTemplateAction>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SendImageCarouselTemplate {
    pub columns: Vec<SendImageCarouselColumn>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SendImageCarouselColumn {
    pub image_url: String,
    pub action: SendTemplateAction,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SendTemplateAction {
    Postback(SendTemplatePostbackAction),
    Message(SendTemplateMessageAction),
    Uri(SendTemplateUriAction),
    Datetimepicker(SendTemplateDatetimepickerAction),
    Camera(SendTemplateCameraAction),
    CameraRoll(SendTemplateCameraRollAction),
    Location(SendTemplateLocationAction),
    Richmenuswitch(SendTemplateRichmenuswitchAction),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SendTemplatePostbackAction {
    pub label: String,
    pub data: String,
    pub display_text: Option<String>,
    pub input_options: Option<String>,
    pub fill_in_text: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SendTemplateMessageAction {
    pub label: String,
    pub text: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SendTemplateUriAction {
    pub label: String,
    pub uri: String,
    pub alt_url: Option<SendTemplateUriActionAltUrl>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SendTemplateUriActionAltUrl {
    pub desktop: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SendTemplateDatetimepickerAction {
    pub label: String,
    pub data: String,
    pub mode: SendTemplateDatetimeMode,
    pub initial: Option<SendTemplateDatetime>,
    pub max: Option<SendTemplateDatetime>,
    pub min: Option<SendTemplateDatetime>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SendTemplateDatetimeMode {
    Date(String),
    Time(String),
    Datetime(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SendTemplateDatetime {
    Date(String),
    Time(String),
    Datetime(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SendTemplateCameraAction {
    pub label: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SendTemplateCameraRollAction {
    pub label: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SendTemplateLocationAction {
    pub label: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SendTemplateRichmenuswitchAction {
    pub label: Option<String>,
    pub rich_menu_alias_id: String,
    pub data: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewSendMessages {
    pub id: Id<SendMessage>,
    pub sending_type: NewSendSendingType,
    pub sending_method: NewSendSendingMethod,
    pub sender: Option<NewSendSender>,
    pub messages: Vec<NewSendMessage>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NewSendSendingType {
    Bot,
    Manual,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NewSendSendingMethod {
    Reply,
    Push,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewSendSender {
    pub id: i64,
    pub name: String,
    pub picture_url: String,
    pub email: String,
    pub sender_role: NewSendSenderRole,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NewSendSenderRole {
    Sender,
}

// TODO Flex Messageの実装
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NewSendMessage {
    Text(NewSendMessageText),
    Sticker(NewSendStickerMessage),
    Image(NewSendImageMessage),
    Video(NewSendVideoMessage),
    Audio(NewSendAudioMessage),
    Location(NewSendLocationMessage),
    Imagemap(NewSendImagemapMessage),
    Template(NewSendTemplateMessage),
}

impl NewSendMessage {
    pub fn created_at(&self) -> &DateTime<Local> {
        match self {
            NewSendMessage::Text(s) => &s.created_at,
            NewSendMessage::Sticker(s) => &s.created_at,
            NewSendMessage::Image(s) => &s.created_at,
            NewSendMessage::Video(s) => &s.created_at,
            NewSendMessage::Audio(s) => &s.created_at,
            NewSendMessage::Location(s) => &s.created_at,
            NewSendMessage::Imagemap(s) => &s.created_at,
            NewSendMessage::Template(s) => &s.created_at,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewSendMessageText {
    pub message_id: String,
    pub text: String,
    pub emojis: Option<Vec<NewSendEmoji>>,
    pub quote_token: Option<NewSendQuoteToken>,
    pub created_at: DateTime<Local>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewSendEmoji {
    pub index: u32,
    pub product_id: String,
    pub emoji_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewSendQuoteToken(pub String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewSendStickerMessage {
    pub message_id: String,
    pub package_id: String,
    pub sticker_id: String,
    pub quote_token: Option<NewSendQuoteToken>,
    pub created_at: DateTime<Local>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewSendImageMessage {
    pub message_id: String,
    pub original_content_url: String,
    pub preview_image_url: String,
    pub created_at: DateTime<Local>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewSendVideoMessage {
    pub message_id: String,
    pub original_content_url: String,
    pub preview_image_url: String,
    pub tracking_id: Option<String>,
    pub created_at: DateTime<Local>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewSendAudioMessage {
    pub message_id: String,
    pub original_content_url: String,
    pub duration: u32,
    pub created_at: DateTime<Local>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewSendLocationMessage {
    pub message_id: String,
    pub title: String,
    pub address: String,
    pub latitude: Decimal,
    pub longitude: Decimal,
    pub created_at: DateTime<Local>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewSendImagemapMessage {
    pub message_id: String,
    pub base_url: String,
    pub alt_text: String,
    pub base_size: NewSendImagemapBaseSize,
    pub video: Option<NewSendImagemapVideo>,
    pub actions: Vec<NewSendImagemapAction>,
    pub created_at: DateTime<Local>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewSendImagemapBaseSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewSendImagemapVideo {
    pub original_content_url: String,
    pub preview_image_url: String,
    pub area: NewSendImagemapVideoArea,
    pub external_link: NewSendImagemapVideoExternalLink,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewSendImagemapVideoArea {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewSendImagemapVideoExternalLink {
    pub link_uri: String,
    pub label: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NewSendImagemapAction {
    Uri(NewSendImagemapUriAction),
    Message(NewSendImagemapMessageAction),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewSendImagemapUriAction {
    pub label: String,
    pub link_uri: String,
    pub area: NewSendImagemapActionArea,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewSendImagemapMessageAction {
    pub label: String,
    pub text: String,
    pub area: NewSendImagemapActionArea,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewSendImagemapActionArea {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewSendTemplateMessage {
    pub message_id: String,
    pub alt_text: String,
    pub template: NewSendTemplateMessageContent,
    pub created_at: DateTime<Local>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NewSendTemplateMessageContent {
    Buttons(NewSendButtonsTemplate),
    Confirm(NewSendConfirmTemplate),
    Carousel(NewSendCarouselTemplate),
    ImageCarousel(NewSendImageCarouselTemplate),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewSendButtonsTemplate {
    pub thumbnail_image_url: Option<String>,
    pub image_aspect_ratio: Option<NewSendImageAspectRatio>,
    pub image_size: Option<NewSendImageSize>,
    pub image_background_color: Option<String>,
    pub title: Option<String>,
    pub text: String,
    pub default_action: Option<NewSendTemplateAction>,
    pub actions: Vec<NewSendTemplateAction>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NewSendImageAspectRatio {
    Rectangle,
    Square,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NewSendImageSize {
    Cover,
    Contain,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewSendConfirmTemplate {
    pub text: String,
    pub actions: Vec<NewSendTemplateAction>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewSendCarouselTemplate {
    pub columns: Vec<NewSendCarouselColumn>,
    pub image_aspect_ratio: Option<NewSendImageAspectRatio>,
    pub image_size: Option<NewSendImageSize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewSendCarouselColumn {
    pub thumbnail_image_url: Option<String>,
    pub image_background_color: Option<String>,
    pub title: Option<String>,
    pub text: String,
    pub default_action: Option<NewSendTemplateAction>,
    pub actions: Vec<NewSendTemplateAction>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewSendImageCarouselTemplate {
    pub columns: Vec<NewSendImageCarouselColumn>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewSendImageCarouselColumn {
    pub image_url: String,
    pub action: NewSendTemplateAction,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NewSendTemplateAction {
    Postback(NewSendTemplatePostbackAction),
    Message(NewSendTemplateMessageAction),
    Uri(NewSendTemplateUriAction),
    Datetimepicker(NewSendTemplateDatetimepickerAction),
    Camera(NewSendTemplateCameraAction),
    CameraRoll(NewSendTemplateCameraRollAction),
    Location(NewSendTemplateLocationAction),
    Richmenuswitch(NewSendTemplateRichmenuswitchAction),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewSendTemplatePostbackAction {
    pub label: String,
    pub data: String,
    pub display_text: Option<String>,
    pub input_options: Option<String>,
    pub fill_in_text: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewSendTemplateMessageAction {
    pub label: String,
    pub text: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewSendTemplateUriAction {
    pub label: String,
    pub uri: String,
    pub alt_url: Option<NewSendTemplateUriActionAltUrl>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewSendTemplateUriActionAltUrl {
    pub desktop: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewSendTemplateDatetimepickerAction {
    pub label: String,
    pub data: String,
    pub mode: NewSendTemplateDatetimeMode,
    pub initial: Option<NewSendTemplateDatetime>,
    pub max: Option<NewSendTemplateDatetime>,
    pub min: Option<NewSendTemplateDatetime>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NewSendTemplateDatetimeMode {
    Date(String),
    Time(String),
    Datetime(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NewSendTemplateDatetime {
    Date(String),
    Time(String),
    Datetime(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewSendTemplateCameraAction {
    pub label: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewSendTemplateCameraRollAction {
    pub label: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewSendTemplateLocationAction {
    pub label: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewSendTemplateRichmenuswitchAction {
    pub label: Option<String>,
    pub rich_menu_alias_id: String,
    pub data: String,
}
