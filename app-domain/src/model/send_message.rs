use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

// TODO Flex Messageの実装
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum SendMessage {
    Text(SendTextMessage),
    Sticker(SendStickerMessage),
    Image(SendImageMessage),
    Video(SendVideoMessage),
    Audio(SendAudioMessage),
    Location(SendLocationMessage),
    Imagemap(SendImagemapMessage),
    Template(SendTemplateMessage),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendTextMessage {
    pub text: String,
    pub emojis: Option<Vec<SendEmoji>>,
    pub quote_token: Option<SendQuoteToken>,
    pub created_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendEmoji {
    pub index: u32,
    pub product_id: String,
    pub emoji_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SendQuoteToken(pub String);

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendStickerMessage {
    pub package_id: String,
    pub sticker_id: String,
    pub quote_token: Option<SendQuoteToken>,
    pub created_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendImageMessage {
    pub original_content_url: String,
    pub preview_image_url: String,
    pub created_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendVideoMessage {
    pub original_content_url: String,
    pub preview_image_url: String,
    pub tracking_id: Option<String>,
    pub created_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendAudioMessage {
    pub original_content_url: String,
    pub duration: u32,
    pub created_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendLocationMessage {
    pub title: String,
    pub address: String,
    pub latitude: f64,
    pub longitude: f64,
    pub created_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendImagemapMessage {
    pub base_url: String,
    pub alt_text: String,
    pub base_size: SendImagemapBaseSize,
    pub video: Option<SendImagemapVideo>,
    pub actions: Vec<SendImagemapAction>,
    pub created_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendImagemapBaseSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendImagemapVideo {
    pub original_content_url: String,
    pub preview_image_url: String,
    pub area: SendImagemapVideoArea,
    pub external_link: SendImagemapVideoExternalLink,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendImagemapVideoArea {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendImagemapVideoExternalLink {
    pub link_uri: String,
    pub label: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SendImagemapAction {
    Uri(SendImagemapUriAction),
    Message(SendImagemapMessageAction),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub struct SendImagemapUriAction {
    pub label: String,
    pub link_uri: String,
    pub area: SendImagemapActionArea,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendImagemapMessageAction {
    pub label: String,
    pub text: String,
    pub area: SendImagemapActionArea,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendImagemapActionArea {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendTemplateMessage {
    pub alt_text: String,
    pub template: SendTemplateMessageContent,
    pub created_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum SendTemplateMessageContent {
    Buttons(SendButtonsTemplate),
    Confirm(SendConfirmTemplate),
    Carousel(SendCarouselTemplate),
    ImageCarousel(SendImageCarouselTemplate),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SendImageAspectRatio {
    Rectangle,
    Square,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SendImageSize {
    Cover,
    Contain,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SendConfirmTemplate {
    pub text: String,
    pub actions: Vec<SendTemplateAction>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SendCarouselTemplate {
    pub columns: Vec<SendCarouselColumn>,
    pub image_aspect_ratio: Option<SendImageAspectRatio>,
    pub image_size: Option<SendImageSize>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendCarouselColumn {
    pub thumbnail_image_url: Option<String>,
    pub image_background_color: Option<String>,
    pub title: Option<String>,
    pub text: String,
    pub default_action: Option<SendTemplateAction>,
    pub actions: Vec<SendTemplateAction>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendImageCarouselTemplate {
    pub columns: Vec<SendImageCarouselColumn>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendImageCarouselColumn {
    pub image_url: String,
    pub action: SendTemplateAction,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendTemplatePostbackAction {
    pub label: String,
    pub data: String,
    pub display_text: Option<String>,
    pub input_options: Option<String>,
    pub fill_in_text: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendTemplateMessageAction {
    pub label: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendTemplateUriAction {
    pub label: String,
    pub uri: String,
    pub alt_url: Option<SendTemplateUriActionAltUrl>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendTemplateUriActionAltUrl {
    pub desktop: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendTemplateDatetimepickerAction {
    pub label: String,
    pub data: String,
    pub mode: SendTemplateDatetimeMode,
    pub initial: Option<SendTemplateDatetime>,
    pub max: Option<SendTemplateDatetime>,
    pub min: Option<SendTemplateDatetime>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SendTemplateDatetimeMode {
    Date(String),
    Time(String),
    Datetime(String),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SendTemplateDatetime {
    Date(String),
    Time(String),
    Datetime(String),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SendTemplateCameraAction {
    pub label: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SendTemplateCameraRollAction {
    pub label: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SendTemplateLocationAction {
    pub label: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendTemplateRichmenuswitchAction {
    pub label: Option<String>,
    pub rich_menu_alias_id: String,
    pub data: String,
}

// TODO Flex Messageの実装
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum NewSendMessage {
    Text(NewSendTextMessage),
    Sticker(NewSendStickerMessage),
    Image(NewSendImageMessage),
    Video(NewSendVideoMessage),
    Audio(NewSendAudioMessage),
    Location(NewSendLocationMessage),
    Imagemap(NewSendImagemapMessage),
    Template(NewSendTemplateMessage),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewSendTextMessage {
    pub id: String,
    pub text: String,
    pub emojis: Option<Vec<NewSendEmoji>>,
    pub quote_token: Option<NewSendQuoteToken>,
    pub created_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewSendEmoji {
    pub index: u32,
    pub product_id: String,
    pub emoji_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewSendQuoteToken(pub String);

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewSendStickerMessage {
    pub id: String,
    pub package_id: String,
    pub sticker_id: String,
    pub quote_token: Option<NewSendQuoteToken>,
    pub created_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewSendImageMessage {
    pub id: String,
    pub original_content_url: String,
    pub preview_image_url: String,
    pub created_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewSendVideoMessage {
    pub id: String,
    pub original_content_url: String,
    pub preview_image_url: String,
    pub tracking_id: Option<String>,
    pub created_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewSendAudioMessage {
    pub id: String,
    pub original_content_url: String,
    pub duration: u32,
    pub created_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewSendLocationMessage {
    pub id: String,
    pub title: String,
    pub address: String,
    pub latitude: f64,
    pub longitude: f64,
    pub created_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewSendImagemapMessage {
    pub id: String,
    pub base_url: String,
    pub alt_text: String,
    pub base_size: NewSendImagemapBaseSize,
    pub video: Option<NewSendImagemapVideo>,
    pub actions: Vec<NewSendImagemapAction>,
    pub created_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewSendImagemapBaseSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewSendImagemapVideo {
    pub original_content_url: String,
    pub preview_image_url: String,
    pub area: NewSendImagemapVideoArea,
    pub external_link: NewSendImagemapVideoExternalLink,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewSendImagemapVideoArea {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewSendImagemapVideoExternalLink {
    pub link_uri: String,
    pub label: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum NewSendImagemapAction {
    Uri(NewSendImagemapUriAction),
    Message(NewSendImagemapMessageAction),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub struct NewSendImagemapUriAction {
    pub label: String,
    pub link_uri: String,
    pub area: NewSendImagemapActionArea,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewSendImagemapMessageAction {
    pub label: String,
    pub text: String,
    pub area: NewSendImagemapActionArea,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewSendImagemapActionArea {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewSendTemplateMessage {
    pub id: String,
    pub alt_text: String,
    pub template: NewSendTemplateMessageContent,
    pub created_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum NewSendTemplateMessageContent {
    Buttons(NewSendButtonsTemplate),
    Confirm(NewSendConfirmTemplate),
    Carousel(NewSendCarouselTemplate),
    ImageCarousel(NewSendImageCarouselTemplate),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum NewSendImageAspectRatio {
    Rectangle,
    Square,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum NewSendImageSize {
    Cover,
    Contain,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewSendConfirmTemplate {
    pub text: String,
    pub actions: Vec<NewSendTemplateAction>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewSendCarouselTemplate {
    pub columns: Vec<NewSendCarouselColumn>,
    pub image_aspect_ratio: Option<NewSendImageAspectRatio>,
    pub image_size: Option<NewSendImageSize>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewSendCarouselColumn {
    pub thumbnail_image_url: Option<String>,
    pub image_background_color: Option<String>,
    pub title: Option<String>,
    pub text: String,
    pub default_action: Option<NewSendTemplateAction>,
    pub actions: Vec<NewSendTemplateAction>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewSendImageCarouselTemplate {
    pub columns: Vec<NewSendImageCarouselColumn>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewSendImageCarouselColumn {
    pub image_url: String,
    pub action: NewSendTemplateAction,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewSendTemplatePostbackAction {
    pub label: String,
    pub data: String,
    pub display_text: Option<String>,
    pub input_options: Option<String>,
    pub fill_in_text: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewSendTemplateMessageAction {
    pub label: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewSendTemplateUriAction {
    pub label: String,
    pub uri: String,
    pub alt_url: Option<NewSendTemplateUriActionAltUrl>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewSendTemplateUriActionAltUrl {
    pub desktop: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewSendTemplateDatetimepickerAction {
    pub label: String,
    pub data: String,
    pub mode: NewSendTemplateDatetimeMode,
    pub initial: Option<NewSendTemplateDatetime>,
    pub max: Option<NewSendTemplateDatetime>,
    pub min: Option<NewSendTemplateDatetime>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum NewSendTemplateDatetimeMode {
    Date(String),
    Time(String),
    Datetime(String),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum NewSendTemplateDatetime {
    Date(String),
    Time(String),
    Datetime(String),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewSendTemplateCameraAction {
    pub label: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewSendTemplateCameraRollAction {
    pub label: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewSendTemplateLocationAction {
    pub label: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewSendTemplateRichmenuswitchAction {
    pub label: Option<String>,
    pub rich_menu_alias_id: String,
    pub data: String,
}
