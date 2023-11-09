use chrono::{DateTime, Local};
use rust_decimal::{prelude::{FromPrimitive, ToPrimitive}, Decimal};
use serde::{Deserialize, Serialize};

use domain::model::message::send_message::{
    NewSendAudioMessage, NewSendButtonsTemplate, NewSendCarouselColumn, NewSendCarouselTemplate,
    NewSendConfirmTemplate, NewSendEmoji, NewSendImageAspectRatio, NewSendImageCarouselColumn,
    NewSendImageCarouselTemplate, NewSendImageMessage, NewSendImageSize, NewSendImagemapAction,
    NewSendImagemapActionArea, NewSendImagemapBaseSize, NewSendImagemapMessage,
    NewSendImagemapMessageAction, NewSendImagemapUriAction, NewSendImagemapVideo,
    NewSendImagemapVideoArea, NewSendImagemapVideoExternalLink, NewSendLocationMessage,
    NewSendMessage, NewSendMessageText, NewSendQuoteToken, NewSendStickerMessage,
    NewSendTemplateAction, NewSendTemplateCameraAction, NewSendTemplateCameraRollAction,
    NewSendTemplateDatetime, NewSendTemplateDatetimeMode, NewSendTemplateDatetimepickerAction,
    NewSendTemplateLocationAction, NewSendTemplateMessage, NewSendTemplateMessageAction,
    NewSendTemplateMessageContent, NewSendTemplatePostbackAction,
    NewSendTemplateRichmenuswitchAction, NewSendTemplateUriAction, NewSendTemplateUriActionAltUrl,
    NewSendVideoMessage, SendAudioMessage, SendButtonsTemplate, SendCarouselColumn,
    SendCarouselTemplate, SendConfirmTemplate, SendEmoji, SendImageAspectRatio,
    SendImageCarouselColumn, SendImageCarouselTemplate, SendImageMessage, SendImageSize,
    SendImagemapAction, SendImagemapActionArea, SendImagemapBaseSize, SendImagemapMessage,
    SendImagemapMessageAction, SendImagemapUriAction, SendImagemapVideo, SendImagemapVideoArea,
    SendImagemapVideoExternalLink, SendLocationMessage, SendMessage, SendMessageText,
    SendQuoteToken, SendStickerMessage, SendTemplateAction, SendTemplateCameraAction,
    SendTemplateCameraRollAction, SendTemplateDatetime, SendTemplateDatetimeMode,
    SendTemplateDatetimepickerAction, SendTemplateLocationAction, SendTemplateMessage,
    SendTemplateMessageAction, SendTemplateMessageContent, SendTemplatePostbackAction,
    SendTemplateRichmenuswitchAction, SendTemplateUriAction, SendTemplateUriActionAltUrl,
    SendVideoMessage,
};

pub fn message_type() -> String {
    "message".to_string()
}

// TODO Flex Messageの実装
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum SendMessageContentTable {
    Text(SendMessageContentTextTable),
    Sticker(SendMessageContentStickerTable),
    Image(SendMessageContentImageTable),
    Video(SendMessageContentVideoTable),
    Audio(SendMessageContentAudioTable),
    Location(SendMessageContentLocationTable),
    Imagemap(SendMessageContentImagemapTable),
    Template(SendMessageContentTemplateTable),
}

impl From<NewSendMessage> for SendMessageContentTable {
    fn from(s: NewSendMessage) -> Self {
        match s {
            NewSendMessage::Text(r) => SendMessageContentTable::Text(r.into()),
            NewSendMessage::Sticker(r) => SendMessageContentTable::Sticker(r.into()),
            NewSendMessage::Image(r) => SendMessageContentTable::Image(r.into()),
            NewSendMessage::Video(r) => SendMessageContentTable::Video(r.into()),
            NewSendMessage::Audio(r) => SendMessageContentTable::Audio(r.into()),
            NewSendMessage::Location(r) => SendMessageContentTable::Location(r.into()),
            NewSendMessage::Imagemap(r) => SendMessageContentTable::Imagemap(r.into()),
            NewSendMessage::Template(r) => SendMessageContentTable::Template(r.into()),
        }
    }
}

impl From<SendMessageContentTable> for SendMessage {
    fn from(s: SendMessageContentTable) -> Self {
        match s {
            SendMessageContentTable::Text(r) => SendMessage::Text(r.into()),
            SendMessageContentTable::Sticker(r) => SendMessage::Sticker(r.into()),
            SendMessageContentTable::Image(r) => SendMessage::Image(r.into()),
            SendMessageContentTable::Video(r) => SendMessage::Video(r.into()),
            SendMessageContentTable::Audio(r) => SendMessage::Audio(r.into()),
            SendMessageContentTable::Location(r) => SendMessage::Location(r.into()),
            SendMessageContentTable::Imagemap(r) => SendMessage::Imagemap(r.into()),
            SendMessageContentTable::Template(r) => SendMessage::Template(r.into()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendMessageContentTextTable {
    pub id: String,
    pub text: String,
    pub emojis: Option<Vec<SendEmojiTable>>,
    pub quote_token: Option<SendQuoteTokenTable>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl From<NewSendMessageText> for SendMessageContentTextTable {
    fn from(s: NewSendMessageText) -> Self {
        Self {
            id: s.message_id,
            text: s.text,
            emojis: s.emojis.map(|es| {
                es.iter()
                    .map(|e| e.clone().into())
                    .collect::<Vec<SendEmojiTable>>()
            }),
            quote_token: s.quote_token.map(|s| s.into()),
            created_at: s.created_at,
            updated_at: s.created_at,
        }
    }
}

impl From<SendMessageContentTextTable> for SendMessageText {
    fn from(s: SendMessageContentTextTable) -> Self {
        SendMessageText {
            message_id: s.id,
            text: s.text,
            emojis: s.emojis.map(|es| {
                es.iter()
                    .map(|e| e.clone().into())
                    .collect::<Vec<SendEmoji>>()
            }),
            quote_token: s.quote_token.map(|s| s.into()),
            created_at: s.created_at,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendEmojiTable {
    pub index: u32,
    pub product_id: String,
    pub emoji_id: String,
}

impl From<NewSendEmoji> for SendEmojiTable {
    fn from(s: NewSendEmoji) -> Self {
        Self {
            index: s.index,
            product_id: s.product_id,
            emoji_id: s.emoji_id,
        }
    }
}

impl From<SendEmojiTable> for SendEmoji {
    fn from(s: SendEmojiTable) -> Self {
        SendEmoji {
            index: s.index,
            product_id: s.product_id,
            emoji_id: s.emoji_id,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SendQuoteTokenTable(pub String);

impl From<NewSendQuoteToken> for SendQuoteTokenTable {
    fn from(s: NewSendQuoteToken) -> Self {
        SendQuoteTokenTable(s.0)
    }
}

impl From<SendQuoteTokenTable> for SendQuoteToken {
    fn from(s: SendQuoteTokenTable) -> Self {
        SendQuoteToken(s.0)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendMessageContentStickerTable {
    pub id: String,
    pub package_id: String,
    pub sticker_id: String,
    pub quote_token: Option<SendQuoteTokenTable>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl From<NewSendStickerMessage> for SendMessageContentStickerTable {
    fn from(s: NewSendStickerMessage) -> Self {
        Self {
            id: s.message_id,
            package_id: s.package_id,
            sticker_id: s.sticker_id,
            quote_token: s.quote_token.map(|s| s.into()),
            created_at: s.created_at,
            updated_at: s.created_at,
        }
    }
}

impl From<SendMessageContentStickerTable> for SendStickerMessage {
    fn from(s: SendMessageContentStickerTable) -> Self {
        SendStickerMessage {
            message_id: s.id,
            package_id: s.package_id,
            sticker_id: s.sticker_id,
            quote_token: s.quote_token.map(|s| s.into()),
            created_at: s.created_at,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendMessageContentImageTable {
    pub id: String,
    pub original_content_url: String,
    pub preview_image_url: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl From<NewSendImageMessage> for SendMessageContentImageTable {
    fn from(s: NewSendImageMessage) -> Self {
        Self {
            id: s.message_id,
            original_content_url: s.original_content_url,
            preview_image_url: s.preview_image_url,
            created_at: s.created_at,
            updated_at: s.created_at,
        }
    }
}

impl From<SendMessageContentImageTable> for SendImageMessage {
    fn from(s: SendMessageContentImageTable) -> Self {
        SendImageMessage {
            message_id: s.id,
            original_content_url: s.original_content_url,
            preview_image_url: s.preview_image_url,
            created_at: s.created_at,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendMessageContentVideoTable {
    pub id: String,
    pub original_content_url: String,
    pub preview_image_url: String,
    pub tracking_id: Option<String>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl From<NewSendVideoMessage> for SendMessageContentVideoTable {
    fn from(s: NewSendVideoMessage) -> Self {
        Self {
            id: s.message_id,
            original_content_url: s.original_content_url,
            preview_image_url: s.preview_image_url,
            tracking_id: s.tracking_id,
            created_at: s.created_at,
            updated_at: s.created_at,
        }
    }
}

impl From<SendMessageContentVideoTable> for SendVideoMessage {
    fn from(s: SendMessageContentVideoTable) -> Self {
        SendVideoMessage {
            message_id: s.id,
            original_content_url: s.original_content_url,
            preview_image_url: s.preview_image_url,
            tracking_id: s.tracking_id,
            created_at: s.created_at,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendMessageContentAudioTable {
    pub id: String,
    pub original_content_url: String,
    pub duration: u32,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl From<NewSendAudioMessage> for SendMessageContentAudioTable {
    fn from(s: NewSendAudioMessage) -> Self {
        Self {
            id: s.message_id,
            original_content_url: s.original_content_url,
            duration: s.duration,
            created_at: s.created_at,
            updated_at: s.created_at,
        }
    }
}

impl From<SendMessageContentAudioTable> for SendAudioMessage {
    fn from(s: SendMessageContentAudioTable) -> Self {
        SendAudioMessage {
            message_id: s.id,
            original_content_url: s.original_content_url,
            duration: s.duration,
            created_at: s.created_at,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendMessageContentLocationTable {
    pub id: String,
    pub title: String,
    pub address: String,
    pub latitude: f64,
    pub longitude: f64,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl From<NewSendLocationMessage> for SendMessageContentLocationTable {
    fn from(s: NewSendLocationMessage) -> Self {
        Self {
            id: s.message_id,
            title: s.title,
            address: s.address,
            latitude: s
                .latitude
                .to_f64()
                .unwrap_or_else(|| panic!("Failed to convert Decimal {} to f64", s.latitude)),
            longitude: s
                .longitude
                .to_f64()
                .unwrap_or_else(|| panic!("Failed to convert Decimal {} to f64", s.longitude)),
            created_at: s.created_at,
            updated_at: s.created_at,
        }
    }
}

impl From<SendMessageContentLocationTable> for SendLocationMessage {
    fn from(s: SendMessageContentLocationTable) -> Self {
        SendLocationMessage {
            message_id: s.id,
            title: s.title,
            address: s.address,
            latitude: Decimal::from_f64(s.latitude)
                .unwrap_or_else(|| panic!("Failed to convert f64 {} to Decimal", s.latitude)),
            longitude: Decimal::from_f64(s.longitude)
                .unwrap_or_else(|| panic!("Failed to convert f64 {} to Decimal", s.longitude)),
            created_at: s.created_at,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendMessageContentImagemapTable {
    pub id: String,
    pub base_url: String,
    pub alt_text: String,
    pub base_size: SendImagemapBaseSizeTable,
    pub video: Option<SendImagemapVideoTable>,
    pub actions: Vec<SendImagemapActionTable>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl From<NewSendImagemapMessage> for SendMessageContentImagemapTable {
    fn from(s: NewSendImagemapMessage) -> Self {
        Self {
            id: s.message_id,
            base_url: s.base_url,
            alt_text: s.alt_text,
            base_size: s.base_size.into(),
            video: s.video.map(|v| v.into()),
            actions: s
                .actions
                .iter()
                .map(|a| a.clone().into())
                .collect::<Vec<SendImagemapActionTable>>(),
            created_at: s.created_at,
            updated_at: s.created_at,
        }
    }
}

impl From<SendMessageContentImagemapTable> for SendImagemapMessage {
    fn from(s: SendMessageContentImagemapTable) -> Self {
        Self {
            message_id: s.id,
            base_url: s.base_url,
            alt_text: s.alt_text,
            base_size: s.base_size.into(),
            video: s.video.map(|v| v.into()),
            actions: s
                .actions
                .iter()
                .map(|a| a.clone().into())
                .collect::<Vec<SendImagemapAction>>(),
            created_at: s.created_at,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendImagemapBaseSizeTable {
    pub width: u32,
    pub height: u32,
}

impl From<NewSendImagemapBaseSize> for SendImagemapBaseSizeTable {
    fn from(s: NewSendImagemapBaseSize) -> Self {
        Self {
            width: s.width,
            height: s.height,
        }
    }
}

impl From<SendImagemapBaseSizeTable> for SendImagemapBaseSize {
    fn from(s: SendImagemapBaseSizeTable) -> Self {
        Self {
            width: s.width,
            height: s.height,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendImagemapVideoTable {
    pub original_content_url: String,
    pub preview_image_url: String,
    pub area: SendImagemapVideoAreaTable,
    pub external_link: SendImagemapVideoExternalLinkTable,
}

impl From<NewSendImagemapVideo> for SendImagemapVideoTable {
    fn from(s: NewSendImagemapVideo) -> Self {
        Self {
            original_content_url: s.original_content_url,
            preview_image_url: s.preview_image_url,
            area: s.area.into(),
            external_link: s.external_link.into(),
        }
    }
}

impl From<SendImagemapVideoTable> for SendImagemapVideo {
    fn from(s: SendImagemapVideoTable) -> Self {
        Self {
            original_content_url: s.original_content_url,
            preview_image_url: s.preview_image_url,
            area: s.area.into(),
            external_link: s.external_link.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendImagemapVideoAreaTable {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl From<NewSendImagemapVideoArea> for SendImagemapVideoAreaTable {
    fn from(s: NewSendImagemapVideoArea) -> Self {
        Self {
            x: s.x,
            y: s.y,
            width: s.width,
            height: s.height,
        }
    }
}

impl From<SendImagemapVideoAreaTable> for SendImagemapVideoArea {
    fn from(s: SendImagemapVideoAreaTable) -> Self {
        Self {
            x: s.x,
            y: s.y,
            width: s.width,
            height: s.height,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendImagemapVideoExternalLinkTable {
    pub link_uri: String,
    pub label: String,
}

impl From<NewSendImagemapVideoExternalLink> for SendImagemapVideoExternalLinkTable {
    fn from(s: NewSendImagemapVideoExternalLink) -> Self {
        Self {
            link_uri: s.link_uri,
            label: s.label,
        }
    }
}

impl From<SendImagemapVideoExternalLinkTable> for SendImagemapVideoExternalLink {
    fn from(s: SendImagemapVideoExternalLinkTable) -> Self {
        Self {
            link_uri: s.link_uri,
            label: s.label,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SendImagemapActionTable {
    Uri(SendImagemapUriActionTable),
    Message(SendImagemapMessageActionTable),
}

impl From<NewSendImagemapAction> for SendImagemapActionTable {
    fn from(s: NewSendImagemapAction) -> Self {
        match s {
            NewSendImagemapAction::Uri(r) => SendImagemapActionTable::Uri(r.into()),
            NewSendImagemapAction::Message(r) => SendImagemapActionTable::Message(r.into()),
        }
    }
}

impl From<SendImagemapActionTable> for SendImagemapAction {
    fn from(s: SendImagemapActionTable) -> Self {
        match s {
            SendImagemapActionTable::Uri(r) => SendImagemapAction::Uri(r.into()),
            SendImagemapActionTable::Message(r) => SendImagemapAction::Message(r.into()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub struct SendImagemapUriActionTable {
    pub label: String,
    pub link_uri: String,
    pub area: SendImagemapActionAreaTable,
}

impl From<NewSendImagemapUriAction> for SendImagemapUriActionTable {
    fn from(s: NewSendImagemapUriAction) -> Self {
        Self {
            label: s.label,
            link_uri: s.link_uri,
            area: s.area.into(),
        }
    }
}

impl From<SendImagemapUriActionTable> for SendImagemapUriAction {
    fn from(s: SendImagemapUriActionTable) -> Self {
        Self {
            label: s.label,
            link_uri: s.link_uri,
            area: s.area.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendImagemapMessageActionTable {
    pub label: String,
    pub text: String,
    pub area: SendImagemapActionAreaTable,
}

impl From<NewSendImagemapMessageAction> for SendImagemapMessageActionTable {
    fn from(s: NewSendImagemapMessageAction) -> Self {
        Self {
            label: s.label,
            text: s.text,
            area: s.area.into(),
        }
    }
}

impl From<SendImagemapMessageActionTable> for SendImagemapMessageAction {
    fn from(s: SendImagemapMessageActionTable) -> Self {
        Self {
            label: s.label,
            text: s.text,
            area: s.area.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendImagemapActionAreaTable {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl From<NewSendImagemapActionArea> for SendImagemapActionAreaTable {
    fn from(s: NewSendImagemapActionArea) -> Self {
        Self {
            x: s.x,
            y: s.y,
            width: s.width,
            height: s.height,
        }
    }
}

impl From<SendImagemapActionAreaTable> for SendImagemapActionArea {
    fn from(s: SendImagemapActionAreaTable) -> Self {
        Self {
            x: s.x,
            y: s.y,
            width: s.width,
            height: s.height,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendMessageContentTemplateTable {
    pub id: String,
    pub alt_text: String,
    pub template: SendTemplateMessageContentTable,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl From<NewSendTemplateMessage> for SendMessageContentTemplateTable {
    fn from(s: NewSendTemplateMessage) -> Self {
        Self {
            id: s.message_id,
            alt_text: s.alt_text,
            template: s.template.into(),
            created_at: s.created_at,
            updated_at: s.created_at,
        }
    }
}

impl From<SendMessageContentTemplateTable> for SendTemplateMessage {
    fn from(s: SendMessageContentTemplateTable) -> Self {
        Self {
            message_id: s.id,
            alt_text: s.alt_text,
            template: s.template.into(),
            created_at: s.created_at,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum SendTemplateMessageContentTable {
    Buttons(SendButtonsTemplateTable),
    Confirm(SendConfirmTemplateTable),
    Carousel(SendCarouselTemplateTable),
    ImageCarousel(SendImageCarouselTemplateTable),
}

impl From<NewSendTemplateMessageContent> for SendTemplateMessageContentTable {
    fn from(s: NewSendTemplateMessageContent) -> Self {
        match s {
            NewSendTemplateMessageContent::Buttons(r) => {
                SendTemplateMessageContentTable::Buttons(r.into())
            }
            NewSendTemplateMessageContent::Confirm(r) => {
                SendTemplateMessageContentTable::Confirm(r.into())
            }
            NewSendTemplateMessageContent::Carousel(r) => {
                SendTemplateMessageContentTable::Carousel(r.into())
            }
            NewSendTemplateMessageContent::ImageCarousel(r) => {
                SendTemplateMessageContentTable::ImageCarousel(r.into())
            }
        }
    }
}

impl From<SendTemplateMessageContentTable> for SendTemplateMessageContent {
    fn from(s: SendTemplateMessageContentTable) -> Self {
        match s {
            SendTemplateMessageContentTable::Buttons(r) => {
                SendTemplateMessageContent::Buttons(r.into())
            }
            SendTemplateMessageContentTable::Confirm(r) => {
                SendTemplateMessageContent::Confirm(r.into())
            }
            SendTemplateMessageContentTable::Carousel(r) => {
                SendTemplateMessageContent::Carousel(r.into())
            }
            SendTemplateMessageContentTable::ImageCarousel(r) => {
                SendTemplateMessageContent::ImageCarousel(r.into())
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendButtonsTemplateTable {
    pub thumbnail_image_url: Option<String>,
    pub image_aspect_ratio: Option<SendImageAspectRatioTable>,
    pub image_size: Option<SendImageSizeTable>,
    pub image_background_color: Option<String>,
    pub title: Option<String>,
    pub text: String,
    pub default_action: Option<SendTemplateActionTable>,
    pub actions: Vec<SendTemplateActionTable>,
}

impl From<NewSendButtonsTemplate> for SendButtonsTemplateTable {
    fn from(s: NewSendButtonsTemplate) -> Self {
        Self {
            thumbnail_image_url: s.thumbnail_image_url,
            image_aspect_ratio: s.image_aspect_ratio.map(|s| s.into()),
            image_size: s.image_size.map(|s| s.into()),
            image_background_color: s.image_background_color,
            title: s.title,
            text: s.text,
            default_action: s.default_action.map(|s| s.into()),
            actions: s
                .actions
                .iter()
                .map(|a| a.clone().into())
                .collect::<Vec<SendTemplateActionTable>>(),
        }
    }
}

impl From<SendButtonsTemplateTable> for SendButtonsTemplate {
    fn from(s: SendButtonsTemplateTable) -> Self {
        Self {
            thumbnail_image_url: s.thumbnail_image_url,
            image_aspect_ratio: s.image_aspect_ratio.map(|s| s.into()),
            image_size: s.image_size.map(|s| s.into()),
            image_background_color: s.image_background_color,
            title: s.title,
            text: s.text,
            default_action: s.default_action.map(|s| s.into()),
            actions: s
                .actions
                .iter()
                .map(|a| a.clone().into())
                .collect::<Vec<SendTemplateAction>>(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SendImageAspectRatioTable {
    Rectangle,
    Square,
}

impl From<NewSendImageAspectRatio> for SendImageAspectRatioTable {
    fn from(s: NewSendImageAspectRatio) -> Self {
        match s {
            NewSendImageAspectRatio::Rectangle => SendImageAspectRatioTable::Rectangle,
            NewSendImageAspectRatio::Square => SendImageAspectRatioTable::Square,
        }
    }
}

impl From<SendImageAspectRatioTable> for SendImageAspectRatio {
    fn from(s: SendImageAspectRatioTable) -> Self {
        match s {
            SendImageAspectRatioTable::Rectangle => SendImageAspectRatio::Rectangle,
            SendImageAspectRatioTable::Square => SendImageAspectRatio::Square,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SendImageSizeTable {
    Cover,
    Contain,
}

impl From<NewSendImageSize> for SendImageSizeTable {
    fn from(s: NewSendImageSize) -> Self {
        match s {
            NewSendImageSize::Cover => SendImageSizeTable::Cover,
            NewSendImageSize::Contain => SendImageSizeTable::Contain,
        }
    }
}

impl From<SendImageSizeTable> for SendImageSize {
    fn from(s: SendImageSizeTable) -> Self {
        match s {
            SendImageSizeTable::Cover => SendImageSize::Cover,
            SendImageSizeTable::Contain => SendImageSize::Contain,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SendConfirmTemplateTable {
    pub text: String,
    pub actions: Vec<SendTemplateActionTable>,
}

impl From<NewSendConfirmTemplate> for SendConfirmTemplateTable {
    fn from(s: NewSendConfirmTemplate) -> Self {
        Self {
            text: s.text,
            actions: s
                .actions
                .iter()
                .map(|a| a.clone().into())
                .collect::<Vec<SendTemplateActionTable>>(),
        }
    }
}

impl From<SendConfirmTemplateTable> for SendConfirmTemplate {
    fn from(s: SendConfirmTemplateTable) -> Self {
        Self {
            text: s.text,
            actions: s
                .actions
                .iter()
                .map(|a| a.clone().into())
                .collect::<Vec<SendTemplateAction>>(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SendCarouselTemplateTable {
    pub columns: Vec<SendCarouselColumnTable>,
    pub image_aspect_ratio: Option<SendImageAspectRatioTable>,
    pub image_size: Option<SendImageSizeTable>,
}

impl From<NewSendCarouselTemplate> for SendCarouselTemplateTable {
    fn from(s: NewSendCarouselTemplate) -> Self {
        Self {
            columns: s
                .columns
                .iter()
                .map(|c| c.clone().into())
                .collect::<Vec<SendCarouselColumnTable>>(),
            image_aspect_ratio: s.image_aspect_ratio.map(|s| s.into()),
            image_size: s.image_size.map(|s| s.into()),
        }
    }
}

impl From<SendCarouselTemplateTable> for SendCarouselTemplate {
    fn from(s: SendCarouselTemplateTable) -> Self {
        Self {
            columns: s
                .columns
                .iter()
                .map(|c| c.clone().into())
                .collect::<Vec<SendCarouselColumn>>(),
            image_aspect_ratio: s.image_aspect_ratio.map(|s| s.into()),
            image_size: s.image_size.map(|s| s.into()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendCarouselColumnTable {
    pub thumbnail_image_url: Option<String>,
    pub image_background_color: Option<String>,
    pub title: Option<String>,
    pub text: String,
    pub default_action: Option<SendTemplateActionTable>,
    pub actions: Vec<SendTemplateActionTable>,
}

impl From<NewSendCarouselColumn> for SendCarouselColumnTable {
    fn from(s: NewSendCarouselColumn) -> Self {
        Self {
            thumbnail_image_url: s.thumbnail_image_url,
            image_background_color: s.image_background_color,
            title: s.title,
            text: s.text,
            default_action: s.default_action.map(|s| s.into()),
            actions: s
                .actions
                .iter()
                .map(|a| a.clone().into())
                .collect::<Vec<SendTemplateActionTable>>(),
        }
    }
}

impl From<SendCarouselColumnTable> for SendCarouselColumn {
    fn from(s: SendCarouselColumnTable) -> Self {
        Self {
            thumbnail_image_url: s.thumbnail_image_url,
            image_background_color: s.image_background_color,
            title: s.title,
            text: s.text,
            default_action: s.default_action.map(|s| s.into()),
            actions: s
                .actions
                .iter()
                .map(|a| a.clone().into())
                .collect::<Vec<SendTemplateAction>>(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendImageCarouselTemplateTable {
    pub columns: Vec<SendImageCarouselColumnTable>,
}

impl From<NewSendImageCarouselTemplate> for SendImageCarouselTemplateTable {
    fn from(s: NewSendImageCarouselTemplate) -> Self {
        Self {
            columns: s
                .columns
                .iter()
                .map(|c| c.clone().into())
                .collect::<Vec<SendImageCarouselColumnTable>>(),
        }
    }
}

impl From<SendImageCarouselTemplateTable> for SendImageCarouselTemplate {
    fn from(s: SendImageCarouselTemplateTable) -> Self {
        Self {
            columns: s
                .columns
                .iter()
                .map(|c| c.clone().into())
                .collect::<Vec<SendImageCarouselColumn>>(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendImageCarouselColumnTable {
    pub image_url: String,
    pub action: SendTemplateActionTable,
}

impl From<NewSendImageCarouselColumn> for SendImageCarouselColumnTable {
    fn from(s: NewSendImageCarouselColumn) -> Self {
        Self {
            image_url: s.image_url,
            action: s.action.into(),
        }
    }
}

impl From<SendImageCarouselColumnTable> for SendImageCarouselColumn {
    fn from(s: SendImageCarouselColumnTable) -> Self {
        Self {
            image_url: s.image_url,
            action: s.action.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum SendTemplateActionTable {
    Postback(SendTemplatePostbackActionTable),
    Message(SendTemplateMessageActionTable),
    Uri(SendTemplateUriActionTable),
    Datetimepicker(SendTemplateDatetimepickerActionTable),
    Camera(SendTemplateCameraActionTable),
    CameraRoll(SendTemplateCameraRollActionTable),
    Location(SendTemplateLocationActionTable),
    Richmenuswitch(SendTemplateRichmenuswitchActionTable),
}

impl From<NewSendTemplateAction> for SendTemplateActionTable {
    fn from(s: NewSendTemplateAction) -> Self {
        match s {
            NewSendTemplateAction::Postback(r) => SendTemplateActionTable::Postback(r.into()),
            NewSendTemplateAction::Message(r) => SendTemplateActionTable::Message(r.into()),
            NewSendTemplateAction::Uri(r) => SendTemplateActionTable::Uri(r.into()),
            NewSendTemplateAction::Datetimepicker(r) => {
                SendTemplateActionTable::Datetimepicker(r.into())
            }
            NewSendTemplateAction::Camera(r) => SendTemplateActionTable::Camera(r.into()),
            NewSendTemplateAction::CameraRoll(r) => SendTemplateActionTable::CameraRoll(r.into()),
            NewSendTemplateAction::Location(r) => SendTemplateActionTable::Location(r.into()),
            NewSendTemplateAction::Richmenuswitch(r) => {
                SendTemplateActionTable::Richmenuswitch(r.into())
            }
        }
    }
}

impl From<SendTemplateActionTable> for SendTemplateAction {
    fn from(s: SendTemplateActionTable) -> Self {
        match s {
            SendTemplateActionTable::Postback(r) => SendTemplateAction::Postback(r.into()),
            SendTemplateActionTable::Message(r) => SendTemplateAction::Message(r.into()),
            SendTemplateActionTable::Uri(r) => SendTemplateAction::Uri(r.into()),
            SendTemplateActionTable::Datetimepicker(r) => {
                SendTemplateAction::Datetimepicker(r.into())
            }
            SendTemplateActionTable::Camera(r) => SendTemplateAction::Camera(r.into()),
            SendTemplateActionTable::CameraRoll(r) => SendTemplateAction::CameraRoll(r.into()),
            SendTemplateActionTable::Location(r) => SendTemplateAction::Location(r.into()),
            SendTemplateActionTable::Richmenuswitch(r) => {
                SendTemplateAction::Richmenuswitch(r.into())
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendTemplatePostbackActionTable {
    pub label: String,
    pub data: String,
    pub display_text: Option<String>,
    pub input_options: Option<String>,
    pub fill_in_text: Option<String>,
}

impl From<NewSendTemplatePostbackAction> for SendTemplatePostbackActionTable {
    fn from(s: NewSendTemplatePostbackAction) -> Self {
        Self {
            label: s.label,
            data: s.data,
            display_text: s.display_text,
            input_options: s.input_options,
            fill_in_text: s.fill_in_text,
        }
    }
}

impl From<SendTemplatePostbackActionTable> for SendTemplatePostbackAction {
    fn from(s: SendTemplatePostbackActionTable) -> Self {
        Self {
            label: s.label,
            data: s.data,
            display_text: s.display_text,
            input_options: s.input_options,
            fill_in_text: s.fill_in_text,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendTemplateMessageActionTable {
    pub label: String,
    pub text: String,
}

impl From<NewSendTemplateMessageAction> for SendTemplateMessageActionTable {
    fn from(s: NewSendTemplateMessageAction) -> Self {
        Self {
            label: s.label,
            text: s.text,
        }
    }
}

impl From<SendTemplateMessageActionTable> for SendTemplateMessageAction {
    fn from(s: SendTemplateMessageActionTable) -> Self {
        Self {
            label: s.label,
            text: s.text,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendTemplateUriActionTable {
    pub label: String,
    pub uri: String,
    pub alt_url: Option<SendTemplateUriActionAltUrlTable>,
}

impl From<NewSendTemplateUriAction> for SendTemplateUriActionTable {
    fn from(s: NewSendTemplateUriAction) -> Self {
        Self {
            label: s.label,
            uri: s.uri,
            alt_url: s.alt_url.map(|s| s.into()),
        }
    }
}

impl From<SendTemplateUriActionTable> for SendTemplateUriAction {
    fn from(s: SendTemplateUriActionTable) -> Self {
        Self {
            label: s.label,
            uri: s.uri,
            alt_url: s.alt_url.map(|s| s.into()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendTemplateUriActionAltUrlTable {
    pub desktop: String,
}

impl From<NewSendTemplateUriActionAltUrl> for SendTemplateUriActionAltUrlTable {
    fn from(s: NewSendTemplateUriActionAltUrl) -> Self {
        Self { desktop: s.desktop }
    }
}

impl From<SendTemplateUriActionAltUrlTable> for SendTemplateUriActionAltUrl {
    fn from(s: SendTemplateUriActionAltUrlTable) -> Self {
        Self { desktop: s.desktop }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendTemplateDatetimepickerActionTable {
    pub label: String,
    pub data: String,
    pub mode: SendTemplateDatetimeModeTable,
    pub initial: Option<SendTemplateDatetimeTable>,
    pub max: Option<SendTemplateDatetimeTable>,
    pub min: Option<SendTemplateDatetimeTable>,
}

impl From<NewSendTemplateDatetimepickerAction> for SendTemplateDatetimepickerActionTable {
    fn from(s: NewSendTemplateDatetimepickerAction) -> Self {
        Self {
            label: s.label,
            data: s.data,
            mode: s.mode.into(),
            initial: s.initial.map(|s| s.into()),
            max: s.max.map(|s| s.into()),
            min: s.min.map(|s| s.into()),
        }
    }
}

impl From<SendTemplateDatetimepickerActionTable> for SendTemplateDatetimepickerAction {
    fn from(s: SendTemplateDatetimepickerActionTable) -> Self {
        Self {
            label: s.label,
            data: s.data,
            mode: s.mode.into(),
            initial: s.initial.map(|s| s.into()),
            max: s.max.map(|s| s.into()),
            min: s.min.map(|s| s.into()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SendTemplateDatetimeModeTable {
    Date(String),
    Time(String),
    Datetime(String),
}

impl From<NewSendTemplateDatetimeMode> for SendTemplateDatetimeModeTable {
    fn from(s: NewSendTemplateDatetimeMode) -> Self {
        match s {
            NewSendTemplateDatetimeMode::Date(s) => SendTemplateDatetimeModeTable::Date(s),
            NewSendTemplateDatetimeMode::Time(s) => SendTemplateDatetimeModeTable::Time(s),
            NewSendTemplateDatetimeMode::Datetime(s) => SendTemplateDatetimeModeTable::Datetime(s),
        }
    }
}

impl From<SendTemplateDatetimeModeTable> for SendTemplateDatetimeMode {
    fn from(s: SendTemplateDatetimeModeTable) -> Self {
        match s {
            SendTemplateDatetimeModeTable::Date(s) => SendTemplateDatetimeMode::Date(s),
            SendTemplateDatetimeModeTable::Time(s) => SendTemplateDatetimeMode::Time(s),
            SendTemplateDatetimeModeTable::Datetime(s) => SendTemplateDatetimeMode::Datetime(s),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SendTemplateDatetimeTable {
    Date(String),
    Time(String),
    Datetime(String),
}

impl From<NewSendTemplateDatetime> for SendTemplateDatetimeTable {
    fn from(s: NewSendTemplateDatetime) -> Self {
        match s {
            NewSendTemplateDatetime::Date(s) => SendTemplateDatetimeTable::Date(s),
            NewSendTemplateDatetime::Time(s) => SendTemplateDatetimeTable::Time(s),
            NewSendTemplateDatetime::Datetime(s) => SendTemplateDatetimeTable::Datetime(s),
        }
    }
}

impl From<SendTemplateDatetimeTable> for SendTemplateDatetime {
    fn from(s: SendTemplateDatetimeTable) -> Self {
        match s {
            SendTemplateDatetimeTable::Date(s) => SendTemplateDatetime::Date(s),
            SendTemplateDatetimeTable::Time(s) => SendTemplateDatetime::Time(s),
            SendTemplateDatetimeTable::Datetime(s) => SendTemplateDatetime::Datetime(s),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SendTemplateCameraActionTable {
    pub label: String,
}

impl From<NewSendTemplateCameraAction> for SendTemplateCameraActionTable {
    fn from(s: NewSendTemplateCameraAction) -> Self {
        Self { label: s.label }
    }
}

impl From<SendTemplateCameraActionTable> for SendTemplateCameraAction {
    fn from(s: SendTemplateCameraActionTable) -> Self {
        Self { label: s.label }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SendTemplateCameraRollActionTable {
    pub label: String,
}

impl From<NewSendTemplateCameraRollAction> for SendTemplateCameraRollActionTable {
    fn from(s: NewSendTemplateCameraRollAction) -> Self {
        Self { label: s.label }
    }
}

impl From<SendTemplateCameraRollActionTable> for SendTemplateCameraRollAction {
    fn from(s: SendTemplateCameraRollActionTable) -> Self {
        Self { label: s.label }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SendTemplateLocationActionTable {
    pub label: String,
}

impl From<NewSendTemplateLocationAction> for SendTemplateLocationActionTable {
    fn from(s: NewSendTemplateLocationAction) -> Self {
        Self { label: s.label }
    }
}

impl From<SendTemplateLocationActionTable> for SendTemplateLocationAction {
    fn from(s: SendTemplateLocationActionTable) -> Self {
        Self { label: s.label }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendTemplateRichmenuswitchActionTable {
    pub label: Option<String>,
    pub rich_menu_alias_id: String,
    pub data: String,
}

impl From<NewSendTemplateRichmenuswitchAction> for SendTemplateRichmenuswitchActionTable {
    fn from(s: NewSendTemplateRichmenuswitchAction) -> Self {
        Self {
            label: s.label,
            rich_menu_alias_id: s.rich_menu_alias_id,
            data: s.data,
        }
    }
}

impl From<SendTemplateRichmenuswitchActionTable> for SendTemplateRichmenuswitchAction {
    fn from(s: SendTemplateRichmenuswitchActionTable) -> Self {
        Self {
            label: s.label,
            rich_menu_alias_id: s.rich_menu_alias_id,
            data: s.data,
        }
    }
}
