use chrono::Local;
use rust_decimal::{prelude::FromPrimitive, Decimal};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::gateway::LINE_MESSAGE_NUMBER_LIMIT;
use domain::model::{
    message::{
        event::NewEvent,
        send_message::{
            NewSendAudioMessage, NewSendButtonsTemplate, NewSendCarouselColumn,
            NewSendCarouselTemplate, NewSendConfirmTemplate, NewSendEmoji, NewSendImageAspectRatio,
            NewSendImageCarouselColumn, NewSendImageCarouselTemplate, NewSendImageMessage,
            NewSendImageSize, NewSendImagemapAction, NewSendImagemapActionArea,
            NewSendImagemapBaseSize, NewSendImagemapMessage, NewSendImagemapMessageAction,
            NewSendImagemapUriAction, NewSendImagemapVideo, NewSendImagemapVideoArea,
            NewSendImagemapVideoExternalLink, NewSendLocationMessage, NewSendMessage,
            NewSendMessageText, NewSendMessages, NewSendQuoteToken, NewSendSender,
            NewSendSendingMethod, NewSendSendingType, NewSendStickerMessage, NewSendTemplateAction,
            NewSendTemplateCameraAction, NewSendTemplateCameraRollAction, NewSendTemplateDatetime,
            NewSendTemplateDatetimeMode, NewSendTemplateDatetimepickerAction,
            NewSendTemplateLocationAction, NewSendTemplateMessage, NewSendTemplateMessageAction,
            NewSendTemplateMessageContent, NewSendTemplatePostbackAction,
            NewSendTemplateRichmenuswitchAction, NewSendTemplateUriAction,
            NewSendTemplateUriActionAltUrl, NewSendVideoMessage,
        },
    },
    Id,
};

pub enum CreateSendMessage {
    Bot(CreateBotSendMessage),
    Manual(CreateManualSendMessage),
}

impl CreateSendMessage {
    pub fn into_chunked_requests(&self, to: String) -> Vec<SendMessageRequest> {
        match self {
            CreateSendMessage::Bot(r) => r.into_chunked_requests(to),
            CreateSendMessage::Manual(r) => r.into_chunked_requests(to),
        }
    }
    pub fn from_event(event: NewEvent) -> Self {
        match event {
            NewEvent::Follow(e) => {
                let messages: Vec<SendMessageContentRequest> = vec![
                    SendMessageContentRequest::Text(SendMessageContentTextRequest {
                        text: "友達登録ありがとうございます！".to_string(),
                        emojis: None,
                        quote_token: None,
                    }),
                    SendMessageContentRequest::Text(SendMessageContentTextRequest {
                        text: "こんにちは！PharmaXです！！".to_string(),
                        emojis: None,
                        quote_token: None,
                    }),
                ];
                print!("from_event messages:{:?}", &messages);
                CreateSendMessage::Bot(CreateBotSendMessage {
                    reply_token: e.reply_token.to_string(),
                    sending_method: SendSendingMethodRequest::Reply,
                    messages,
                })
            }
            _ => {
                let messages = vec![SendMessageContentRequest::Text(
                    SendMessageContentTextRequest {
                        text: "".to_string(),
                        emojis: None,
                        quote_token: None,
                    },
                )];
                CreateSendMessage::Bot(CreateBotSendMessage {
                    reply_token: "".to_string(),
                    sending_method: SendSendingMethodRequest::Reply,
                    messages,
                })
            }
        }
    }
}
/*
 * Bot
 */
// BotSendMessageRequest は、Botメッセージなのでreplayかpushかは決まっていない
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateBotSendMessage {
    reply_token: String,
    sending_method: SendSendingMethodRequest,
    messages: Vec<SendMessageContentRequest>,
}

impl CreateBotSendMessage {
    pub fn into_chunked_requests(&self, to: String) -> Vec<SendMessageRequest> {
        let chunked_message_contents: Vec<Vec<SendMessageContentRequest>> =
            chunk_request(self.messages.clone());
        match self.sending_method {
            SendSendingMethodRequest::Reply => {
                let reply_request = SendMessageRequest::Reply(ReplySendMessageRequest {
                    reply_token: self.reply_token.clone(),
                    sending_type: SendSendingTypeRequest::Bot,
                    messages: chunked_message_contents
                        .first()
                        .unwrap_or_else(|| panic!("Failed to get chunked_message_contents.first()"))
                        .to_vec(),
                });
                let push_requests = chunked_message_contents[1..]
                    .iter()
                    .map(|chunk| {
                        SendMessageRequest::Push(PushSendMessageRequest::new(
                            to.clone(),
                            SendSendingTypeRequest::Bot,
                            chunk.to_vec(),
                        ))
                    })
                    .collect::<Vec<SendMessageRequest>>();
                [vec![reply_request], push_requests].concat()
            }
            SendSendingMethodRequest::Push => chunked_message_contents
                .iter()
                .map(|chunk| {
                    SendMessageRequest::Push(PushSendMessageRequest::new(
                        to.clone(),
                        SendSendingTypeRequest::Bot,
                        chunk.to_vec(),
                    ))
                })
                .collect::<Vec<SendMessageRequest>>(),
        }
    }
}
fn chunk_request(messages: Vec<SendMessageContentRequest>) -> Vec<Vec<SendMessageContentRequest>> {
    let chunked_message_contents: Vec<_> = messages
        .chunks(LINE_MESSAGE_NUMBER_LIMIT)
        .map(|chunk| chunk.to_vec())
        .collect();
    chunked_message_contents
}

/*
 * Manual
 */
// ManualSendMessageRequest は、手動で送信するメッセージなのでpushと決まっている
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateManualSendMessage {
    to: String,
    sending_method: SendSendingMethodRequest,
    messages: Vec<SendMessageContentRequest>,
}

impl CreateManualSendMessage {
    pub fn into_chunked_requests(&self, to: String) -> Vec<SendMessageRequest> {
        let chunked_message_contents: Vec<Vec<SendMessageContentRequest>> =
            chunk_request(self.messages.clone());
        chunked_message_contents
            .iter()
            .map(|chunk| {
                SendMessageRequest::Push(PushSendMessageRequest::new(
                    to.clone(),
                    SendSendingTypeRequest::Bot,
                    chunk.to_vec(),
                ))
            })
            .collect::<Vec<SendMessageRequest>>()
    }
}

/*
 * 共通
 */
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum SendMessageRequest {
    Reply(ReplySendMessageRequest),
    Push(PushSendMessageRequest),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReplySendMessageRequest {
    pub reply_token: String,
    #[serde(default = "replay_send_sending_type")]
    pub sending_type: SendSendingTypeRequest,
    pub messages: Vec<SendMessageContentRequest>,
}

impl ReplySendMessageRequest {
    pub fn into_messages(
        &self,
        sender: Option<NewSendSender>,
        sent_messages: SentMessagesResponse,
    ) -> NewSendMessages {
        let id = Id::gen();
        let sending_type = self.sending_type.clone().into();
        let sending_method = NewSendSendingMethod::Reply;
        let messages = self
            .messages
            .iter()
            .zip(sent_messages.sent_messages.iter())
            .map(|(send_message_request, sent_message)| {
                send_message_request.into(sent_message.message_id.clone())
            })
            .collect();
        NewSendMessages {
            id,
            sending_type,
            sending_method,
            sender,
            messages,
        }
    }
}

pub fn replay_send_sending_type() -> SendSendingTypeRequest {
    SendSendingTypeRequest::Bot
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PushSendMessageRequest {
    pub to: String,
    pub retry_key: String,
    pub sending_type: SendSendingTypeRequest,
    pub messages: Vec<SendMessageContentRequest>,
}

impl PushSendMessageRequest {
    pub fn new(
        to: String,
        sending_type: SendSendingTypeRequest,
        messages: Vec<SendMessageContentRequest>,
    ) -> Self {
        Self {
            to,
            retry_key: Uuid::new_v4().to_string(),
            sending_type,
            messages,
        }
    }
}

impl PushSendMessageRequest {
    pub fn into_messages(
        &self,
        sender: Option<NewSendSender>,
        sent_messages: SentMessagesResponse,
    ) -> NewSendMessages {
        let id = Id::gen();
        let sending_type = self.sending_type.clone().into();
        let sending_method = NewSendSendingMethod::Push;
        let messages = self
            .messages
            .iter()
            .zip(sent_messages.sent_messages.iter())
            .map(|(send_message_request, sent_message)| {
                send_message_request.into(sent_message.message_id.clone())
            })
            .collect();
        NewSendMessages {
            id,
            sending_type,
            sending_method,
            sender,
            messages,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SendSendingTypeRequest {
    Manual,
    Bot,
}

impl From<SendSendingTypeRequest> for NewSendSendingType {
    fn from(s: SendSendingTypeRequest) -> Self {
        match s {
            SendSendingTypeRequest::Manual => Self::Manual,
            SendSendingTypeRequest::Bot => Self::Bot,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SendSendingMethodRequest {
    Reply,
    Push,
}

// TODO Flex Messageの実装
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum SendMessageContentRequest {
    Text(SendMessageContentTextRequest),
    Sticker(SendMessageContentStickerRequest),
    Image(SendMessageContentImageRequest),
    Video(SendMessageContentVideoRequest),
    Audio(SendMessageContentAudioRequest),
    Location(SendMessageContentLocationRequest),
    Imagemap(SendMessageContentImagemapRequest),
    Template(SendMessageContentTemplateRequest),
}

impl SendMessageContentRequest {
    pub fn into(&self, message_id: String) -> NewSendMessage {
        match self {
            SendMessageContentRequest::Text(r) => NewSendMessage::Text(r.into(message_id)),
            SendMessageContentRequest::Sticker(r) => NewSendMessage::Sticker(r.into(message_id)),
            SendMessageContentRequest::Image(r) => NewSendMessage::Image(r.into(message_id)),
            SendMessageContentRequest::Video(r) => NewSendMessage::Video(r.into(message_id)),
            SendMessageContentRequest::Audio(r) => NewSendMessage::Audio(r.into(message_id)),
            SendMessageContentRequest::Location(r) => NewSendMessage::Location(r.into(message_id)),
            SendMessageContentRequest::Imagemap(r) => NewSendMessage::Imagemap(r.into(message_id)),
            SendMessageContentRequest::Template(r) => NewSendMessage::Template(r.into(message_id)),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendMessageContentTextRequest {
    pub text: String,
    pub emojis: Option<Vec<SendEmojiRequest>>,
    pub quote_token: Option<SendQuoteTokenRequest>,
}

impl SendMessageContentTextRequest {
    pub fn into(&self, message_id: String) -> NewSendMessageText {
        let created_at = Local::now();
        NewSendMessageText {
            message_id,
            text: self.text.clone(),
            emojis: self.emojis.clone().map(|es| {
                es.iter()
                    .map(|e| e.clone().into())
                    .collect::<Vec<NewSendEmoji>>()
            }),
            quote_token: self.quote_token.clone().map(|s| s.into()),
            created_at,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendEmojiRequest {
    pub index: u32,
    pub product_id: String,
    pub emoji_id: String,
}

impl From<SendEmojiRequest> for NewSendEmoji {
    fn from(s: SendEmojiRequest) -> Self {
        Self {
            index: s.index,
            product_id: s.product_id,
            emoji_id: s.emoji_id,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SendQuoteTokenRequest(pub String);

impl From<SendQuoteTokenRequest> for NewSendQuoteToken {
    fn from(s: SendQuoteTokenRequest) -> Self {
        NewSendQuoteToken(s.0)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendMessageContentStickerRequest {
    pub package_id: String,
    pub sticker_id: String,
    pub quote_token: Option<SendQuoteTokenRequest>,
}

impl SendMessageContentStickerRequest {
    fn into(&self, message_id: String) -> NewSendStickerMessage {
        let created_at = Local::now();
        NewSendStickerMessage {
            message_id,
            package_id: self.package_id.clone(),
            sticker_id: self.sticker_id.clone(),
            quote_token: self.clone().quote_token.map(|s| s.into()),
            created_at,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendMessageContentImageRequest {
    pub original_content_url: String,
    pub preview_image_url: String,
}

impl SendMessageContentImageRequest {
    fn into(&self, message_id: String) -> NewSendImageMessage {
        let created_at = Local::now();
        NewSendImageMessage {
            message_id,
            original_content_url: self.original_content_url.clone(),
            preview_image_url: self.preview_image_url.clone(),
            created_at,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendMessageContentVideoRequest {
    pub original_content_url: String,
    pub preview_image_url: String,
    pub tracking_id: Option<String>,
}

impl SendMessageContentVideoRequest {
    fn into(&self, message_id: String) -> NewSendVideoMessage {
        let created_at = Local::now();
        NewSendVideoMessage {
            message_id,
            original_content_url: self.original_content_url.clone(),
            preview_image_url: self.preview_image_url.clone(),
            tracking_id: self.tracking_id.clone(),
            created_at,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendMessageContentAudioRequest {
    pub original_content_url: String,
    pub duration: u32,
}

impl SendMessageContentAudioRequest {
    fn into(&self, message_id: String) -> NewSendAudioMessage {
        let created_at = Local::now();
        NewSendAudioMessage {
            message_id,
            original_content_url: self.original_content_url.clone(),
            duration: self.duration,
            created_at,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendMessageContentLocationRequest {
    pub title: String,
    pub address: String,
    pub latitude: f64,
    pub longitude: f64,
}

impl SendMessageContentLocationRequest {
    fn into(&self, message_id: String) -> NewSendLocationMessage {
        let created_at = Local::now();
        NewSendLocationMessage {
            message_id,
            title: self.title.clone(),
            address: self.address.clone(),
            latitude: Decimal::from_f64(self.latitude)
                .unwrap_or_else(|| panic!("Failed to convert f64 {} to Decimal", self.latitude)),
            longitude: Decimal::from_f64(self.longitude)
                .unwrap_or_else(|| panic!("Failed to convert f64 {} to Decimal", self.longitude)),
            created_at,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendMessageContentImagemapRequest {
    pub base_url: String,
    pub alt_text: String,
    pub base_size: SendImagemapBaseSizeRequest,
    pub video: Option<SendImagemapVideoRequest>,
    pub actions: Vec<SendImagemapActionRequest>,
}

impl SendMessageContentImagemapRequest {
    fn into(&self, message_id: String) -> NewSendImagemapMessage {
        let created_at = Local::now();
        NewSendImagemapMessage {
            message_id,
            base_url: self.base_url.clone(),
            alt_text: self.alt_text.clone(),
            base_size: self.base_size.clone().into(),
            video: self.video.clone().map(|s| s.into()),
            actions: self
                .actions
                .iter()
                .map(|s| s.clone().into())
                .collect::<Vec<NewSendImagemapAction>>(),
            created_at,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendImagemapBaseSizeRequest {
    pub width: u32,
    pub height: u32,
}

impl From<SendImagemapBaseSizeRequest> for NewSendImagemapBaseSize {
    fn from(s: SendImagemapBaseSizeRequest) -> Self {
        Self {
            width: s.width,
            height: s.height,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendImagemapVideoRequest {
    pub original_content_url: String,
    pub preview_image_url: String,
    pub area: SendImagemapVideoAreaRequest,
    pub external_link: SendImagemapVideoExternalLinkRequest,
}

impl From<SendImagemapVideoRequest> for NewSendImagemapVideo {
    fn from(s: SendImagemapVideoRequest) -> Self {
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
pub struct SendImagemapVideoAreaRequest {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl From<SendImagemapVideoAreaRequest> for NewSendImagemapVideoArea {
    fn from(s: SendImagemapVideoAreaRequest) -> Self {
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
pub struct SendImagemapVideoExternalLinkRequest {
    pub link_uri: String,
    pub label: String,
}

impl From<SendImagemapVideoExternalLinkRequest> for NewSendImagemapVideoExternalLink {
    fn from(s: SendImagemapVideoExternalLinkRequest) -> Self {
        Self {
            link_uri: s.link_uri,
            label: s.label,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SendImagemapActionRequest {
    Uri(SendImagemapUriActionRequest),
    Message(SendImagemapMessageActionRequest),
}

impl From<SendImagemapActionRequest> for NewSendImagemapAction {
    fn from(s: SendImagemapActionRequest) -> Self {
        match s {
            SendImagemapActionRequest::Uri(r) => NewSendImagemapAction::Uri(r.into()),
            SendImagemapActionRequest::Message(r) => NewSendImagemapAction::Message(r.into()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub struct SendImagemapUriActionRequest {
    pub label: String,
    pub link_uri: String,
    pub area: SendImagemapActionAreaRequest,
}

impl From<SendImagemapUriActionRequest> for NewSendImagemapUriAction {
    fn from(s: SendImagemapUriActionRequest) -> Self {
        Self {
            label: s.label,
            link_uri: s.link_uri,
            area: s.area.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendImagemapMessageActionRequest {
    pub label: String,
    pub text: String,
    pub area: SendImagemapActionAreaRequest,
}

impl From<SendImagemapMessageActionRequest> for NewSendImagemapMessageAction {
    fn from(s: SendImagemapMessageActionRequest) -> Self {
        Self {
            label: s.label,
            text: s.text,
            area: s.area.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendImagemapActionAreaRequest {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl From<SendImagemapActionAreaRequest> for NewSendImagemapActionArea {
    fn from(s: SendImagemapActionAreaRequest) -> Self {
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
pub struct SendMessageContentTemplateRequest {
    pub alt_text: String,
    pub template: SendTemplateMessageContentRequest,
}

impl SendMessageContentTemplateRequest {
    fn into(&self, message_id: String) -> NewSendTemplateMessage {
        let created_at = Local::now();
        NewSendTemplateMessage {
            message_id,
            alt_text: self.alt_text.clone(),
            template: self.template.clone().into(),
            created_at,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum SendTemplateMessageContentRequest {
    Buttons(SendButtonsTemplateRequest),
    Confirm(SendConfirmTemplateRequest),
    Carousel(SendCarouselTemplateRequest),
    ImageCarousel(SendImageCarouselTemplateRequest),
}

impl From<SendTemplateMessageContentRequest> for NewSendTemplateMessageContent {
    fn from(s: SendTemplateMessageContentRequest) -> Self {
        match s {
            SendTemplateMessageContentRequest::Buttons(r) => {
                NewSendTemplateMessageContent::Buttons(r.into())
            }
            SendTemplateMessageContentRequest::Confirm(r) => {
                NewSendTemplateMessageContent::Confirm(r.into())
            }
            SendTemplateMessageContentRequest::Carousel(r) => {
                NewSendTemplateMessageContent::Carousel(r.into())
            }
            SendTemplateMessageContentRequest::ImageCarousel(r) => {
                NewSendTemplateMessageContent::ImageCarousel(r.into())
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendButtonsTemplateRequest {
    pub thumbnail_image_url: Option<String>,
    pub image_aspect_ratio: Option<SendImageAspectRatioRequest>,
    pub image_size: Option<SendImageSizeRequest>,
    pub image_background_color: Option<String>,
    pub title: Option<String>,
    pub text: String,
    pub default_action: Option<SendTemplateActionRequest>,
    pub actions: Vec<SendTemplateActionRequest>,
}

impl From<SendButtonsTemplateRequest> for NewSendButtonsTemplate {
    fn from(s: SendButtonsTemplateRequest) -> Self {
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
                .collect::<Vec<NewSendTemplateAction>>(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SendImageAspectRatioRequest {
    Rectangle,
    Square,
}

impl From<SendImageAspectRatioRequest> for NewSendImageAspectRatio {
    fn from(s: SendImageAspectRatioRequest) -> Self {
        match s {
            SendImageAspectRatioRequest::Rectangle => Self::Rectangle,
            SendImageAspectRatioRequest::Square => Self::Square,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SendImageSizeRequest {
    Cover,
    Contain,
}

impl From<SendImageSizeRequest> for NewSendImageSize {
    fn from(s: SendImageSizeRequest) -> Self {
        match s {
            SendImageSizeRequest::Cover => Self::Cover,
            SendImageSizeRequest::Contain => Self::Contain,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SendConfirmTemplateRequest {
    pub text: String,
    pub actions: Vec<SendTemplateActionRequest>,
}

impl From<SendConfirmTemplateRequest> for NewSendConfirmTemplate {
    fn from(s: SendConfirmTemplateRequest) -> Self {
        Self {
            text: s.text,
            actions: s
                .actions
                .iter()
                .map(|a| a.clone().into())
                .collect::<Vec<NewSendTemplateAction>>(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SendCarouselTemplateRequest {
    pub columns: Vec<SendCarouselColumnRequest>,
    pub image_aspect_ratio: Option<SendImageAspectRatioRequest>,
    pub image_size: Option<SendImageSizeRequest>,
}

impl From<SendCarouselTemplateRequest> for NewSendCarouselTemplate {
    fn from(s: SendCarouselTemplateRequest) -> Self {
        Self {
            columns: s.columns.iter().map(|c| c.clone().into()).collect(),
            image_aspect_ratio: s.image_aspect_ratio.map(|i| i.into()),
            image_size: s.image_size.map(|i| i.into()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendCarouselColumnRequest {
    pub thumbnail_image_url: Option<String>,
    pub image_background_color: Option<String>,
    pub title: Option<String>,
    pub text: String,
    pub default_action: Option<SendTemplateActionRequest>,
    pub actions: Vec<SendTemplateActionRequest>,
}

impl From<SendCarouselColumnRequest> for NewSendCarouselColumn {
    fn from(s: SendCarouselColumnRequest) -> Self {
        Self {
            thumbnail_image_url: s.thumbnail_image_url,
            image_background_color: s.image_background_color,
            title: s.title,
            text: s.text,
            default_action: s.default_action.map(|i| i.into()),
            actions: s
                .actions
                .iter()
                .map(|s| s.clone().into())
                .collect::<Vec<NewSendTemplateAction>>(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendImageCarouselTemplateRequest {
    pub columns: Vec<SendImageCarouselColumn>,
}

impl From<SendImageCarouselTemplateRequest> for NewSendImageCarouselTemplate {
    fn from(s: SendImageCarouselTemplateRequest) -> Self {
        Self {
            columns: s
                .columns
                .iter()
                .map(|s| s.clone().into())
                .collect::<Vec<NewSendImageCarouselColumn>>(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendImageCarouselColumn {
    pub image_url: String,
    pub action: SendTemplateActionRequest,
}

impl From<SendImageCarouselColumn> for NewSendImageCarouselColumn {
    fn from(s: SendImageCarouselColumn) -> Self {
        Self {
            image_url: s.image_url,
            action: s.action.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum SendTemplateActionRequest {
    Postback(SendTemplatePostbackActionRequest),
    Message(SendTemplateMessageActionRequest),
    Uri(SendTemplateUriActionRequest),
    Datetimepicker(SendTemplateDatetimepickerActionRequest),
    Camera(SendTemplateCameraActionRequest),
    CameraRoll(SendTemplateCameraRollActionRequest),
    Location(SendTemplateLocationActionRequest),
    Richmenuswitch(SendTemplateRichmenuswitchActionRequest),
}

impl From<SendTemplateActionRequest> for NewSendTemplateAction {
    fn from(s: SendTemplateActionRequest) -> Self {
        match s {
            SendTemplateActionRequest::Postback(r) => NewSendTemplateAction::Postback(r.into()),
            SendTemplateActionRequest::Message(r) => NewSendTemplateAction::Message(r.into()),
            SendTemplateActionRequest::Uri(r) => NewSendTemplateAction::Uri(r.into()),
            SendTemplateActionRequest::Datetimepicker(r) => {
                NewSendTemplateAction::Datetimepicker(r.into())
            }
            SendTemplateActionRequest::Camera(r) => NewSendTemplateAction::Camera(r.into()),
            SendTemplateActionRequest::CameraRoll(r) => NewSendTemplateAction::CameraRoll(r.into()),
            SendTemplateActionRequest::Location(r) => NewSendTemplateAction::Location(r.into()),
            SendTemplateActionRequest::Richmenuswitch(r) => {
                NewSendTemplateAction::Richmenuswitch(r.into())
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendTemplatePostbackActionRequest {
    pub label: String,
    pub data: String,
    pub display_text: Option<String>,
    pub input_options: Option<String>,
    pub fill_in_text: Option<String>,
}

impl From<SendTemplatePostbackActionRequest> for NewSendTemplatePostbackAction {
    fn from(s: SendTemplatePostbackActionRequest) -> Self {
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
pub struct SendTemplateMessageActionRequest {
    pub label: String,
    pub text: String,
}

impl From<SendTemplateMessageActionRequest> for NewSendTemplateMessageAction {
    fn from(s: SendTemplateMessageActionRequest) -> Self {
        Self {
            label: s.label,
            text: s.text,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendTemplateUriActionRequest {
    pub label: String,
    pub uri: String,
    pub alt_url: Option<SendTemplateUriActionAltUrlRequest>,
}

impl From<SendTemplateUriActionRequest> for NewSendTemplateUriAction {
    fn from(s: SendTemplateUriActionRequest) -> Self {
        Self {
            label: s.label,
            uri: s.uri,
            alt_url: s.alt_url.map(|a| a.into()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendTemplateUriActionAltUrlRequest {
    pub desktop: String,
}

impl From<SendTemplateUriActionAltUrlRequest> for NewSendTemplateUriActionAltUrl {
    fn from(s: SendTemplateUriActionAltUrlRequest) -> Self {
        Self { desktop: s.desktop }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendTemplateDatetimepickerActionRequest {
    pub label: String,
    pub data: String,
    pub mode: SendTemplateDatetimeModeRequest,
    pub initial: Option<SendTemplateDatetime>,
    pub max: Option<SendTemplateDatetime>,
    pub min: Option<SendTemplateDatetime>,
}

impl From<SendTemplateDatetimepickerActionRequest> for NewSendTemplateDatetimepickerAction {
    fn from(s: SendTemplateDatetimepickerActionRequest) -> Self {
        Self {
            label: s.label,
            data: s.data,
            mode: s.mode.into(),
            initial: s.initial.map(|i| i.into()),
            max: s.max.map(|i| i.into()),
            min: s.min.map(|i| i.into()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SendTemplateDatetimeModeRequest {
    Date(String),
    Time(String),
    Datetime(String),
}

impl From<SendTemplateDatetimeModeRequest> for NewSendTemplateDatetimeMode {
    fn from(s: SendTemplateDatetimeModeRequest) -> Self {
        match s {
            SendTemplateDatetimeModeRequest::Date(s) => Self::Date(s),
            SendTemplateDatetimeModeRequest::Time(s) => Self::Time(s),
            SendTemplateDatetimeModeRequest::Datetime(s) => Self::Datetime(s),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SendTemplateDatetime {
    Date(String),
    Time(String),
    Datetime(String),
}

impl From<SendTemplateDatetime> for NewSendTemplateDatetime {
    fn from(s: SendTemplateDatetime) -> Self {
        match s {
            SendTemplateDatetime::Date(s) => Self::Date(s),
            SendTemplateDatetime::Time(s) => Self::Time(s),
            SendTemplateDatetime::Datetime(s) => Self::Datetime(s),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SendTemplateCameraActionRequest {
    pub label: String,
}

impl From<SendTemplateCameraActionRequest> for NewSendTemplateCameraAction {
    fn from(s: SendTemplateCameraActionRequest) -> Self {
        Self { label: s.label }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SendTemplateCameraRollActionRequest {
    pub label: String,
}

impl From<SendTemplateCameraRollActionRequest> for NewSendTemplateCameraRollAction {
    fn from(s: SendTemplateCameraRollActionRequest) -> Self {
        Self { label: s.label }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SendTemplateLocationActionRequest {
    pub label: String,
}

impl From<SendTemplateLocationActionRequest> for NewSendTemplateLocationAction {
    fn from(s: SendTemplateLocationActionRequest) -> Self {
        Self { label: s.label }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendTemplateRichmenuswitchActionRequest {
    pub label: Option<String>,
    pub rich_menu_alias_id: String,
    pub data: String,
}

impl From<SendTemplateRichmenuswitchActionRequest> for NewSendTemplateRichmenuswitchAction {
    fn from(s: SendTemplateRichmenuswitchActionRequest) -> Self {
        Self {
            label: s.label,
            rich_menu_alias_id: s.rich_menu_alias_id,
            data: s.data,
        }
    }
}

/*
* Response
* メッセージ送信のAPIのレスポンスをDeserializeする用
* LINEのメッセージ送信APIはすべてステータス200コードでJSON形式で返ってくるので、この形で返って来ないときにエラーが起きていたと判断できる
*/
#[derive(Deserialize, Clone, Debug)]
pub struct SentMessageResponse {
    #[serde(rename(deserialize = "id"))]
    pub message_id: String,
    #[serde(rename(deserialize = "quoteToken"))]
    pub quote_token: Option<SendQuoteTokenRequest>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SentMessagesResponse {
    pub sent_messages: Vec<SentMessageResponse>,
}
