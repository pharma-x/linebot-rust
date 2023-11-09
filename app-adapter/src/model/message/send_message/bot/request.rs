use serde::{Deserialize, Serialize};

use crate::model::message::send_message::request::{
    SendMessageContentRequest, SendMessageContentTextRequest, SentMessagesResponse,
};
use domain::model::{
    message::{event::NewEvent, send_message::NewSendMessages},
    Id,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BotSendMessageRequest {
    reply_token: String,
    messages: Vec<SendMessageContentRequest>,
}

impl BotSendMessageRequest {
    pub fn into_messages(&self, sent_messages: SentMessagesResponse) -> NewSendMessages {
        // `zip` を使用して、`self.messages` と `sent_messages.sent_messages` の各要素を組み合わせます。
        let id = Id::gen();
        let messages = self
            .messages
            .iter()
            .zip(sent_messages.sent_messages.iter())
            .map(|(send_message_request, sent_message)| {
                send_message_request.into(sent_message.message_id.clone())
            })
            .collect();
        NewSendMessages { id, messages }
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
                Self {
                    reply_token: e.reply_token.to_string(),
                    messages,
                }
            }
            _ => {
                let messages = vec![SendMessageContentRequest::Text(
                    SendMessageContentTextRequest {
                        text: "".to_string(),
                        emojis: None,
                        quote_token: None,
                    },
                )];
                print!("from_event messages:{:?}", &messages);
                Self {
                    reply_token: "".to_string(),
                    messages,
                }
            }
        }
    }
}
