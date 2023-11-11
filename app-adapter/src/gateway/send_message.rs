use anyhow::anyhow;
use async_trait::async_trait;
use futures::future;
use reqwest::header;

use crate::{
    gateway::{GatewayError, HttpClientRepositoryImpl},
    model::message::send_message::request::{
        CreateSendMessage, PushSendMessageRequest, ReplySendMessageRequest, SendMessageRequest,
        SentMessagesResponse,
    },
};
use domain::{
    gateway::send_message::SendMessageGateway,
    model::{
        message::{
            event::NewEvent,
            send_message::{NewSendMessages, NewSendSender, SendMessage},
        },
        user_auth::{LineAuthToken, UserAuthData},
    },
};

#[async_trait]
impl SendMessageGateway for HttpClientRepositoryImpl<SendMessage> {
    async fn send_messages(
        &self,
        user_auth_data: UserAuthData,
        sender: Option<NewSendSender>,
        event: NewEvent,
    ) -> anyhow::Result<Vec<NewSendMessages>> {
        let messages = match user_auth_data {
            UserAuthData::Line(line_user_auth) => {
                /*
                 * lineのメッセージを作成する
                 */
                // todo 作成は上のレイヤーでする
                let bot_message = CreateSendMessage::from_event(event);

                let requests = bot_message.into_chunked_requests(line_user_auth.auth_id.0);
                self.send_line_messages(line_user_auth.auth_token, sender, requests)
                    .await?
            }
        };
        Ok(messages)
    }
}

impl HttpClientRepositoryImpl<SendMessage> {
    async fn send_line_messages(
        &self,
        auth_token: LineAuthToken,
        sender: Option<NewSendSender>,
        message_requests: Vec<SendMessageRequest>,
    ) -> anyhow::Result<Vec<NewSendMessages>> {
        let mut new_messages_vec = Vec::new();
        // メッセージのリクエストの順番を保つ必要があるので、同期処理にした
        for message_request in message_requests {
            let new_message = match message_request {
                SendMessageRequest::Reply(message_request) => {
                    self.send_line_reply_messages(
                        auth_token.clone(),
                        sender.clone(),
                        message_request,
                    )
                    .await?
                }
                SendMessageRequest::Push(message_request) => {
                    self.send_line_push_messages(
                        auth_token.clone(),
                        sender.clone(),
                        message_request,
                    )
                    .await?
                }
            };
            new_messages_vec.push(new_message);
        }
        Ok(new_messages_vec)
    }
    async fn send_line_reply_messages(
        &self,
        auth_token: LineAuthToken,
        sender: Option<NewSendSender>,
        message_request: ReplySendMessageRequest,
    ) -> anyhow::Result<NewSendMessages> {
        let body = self
            .client
            .post("https://api.line.me/v2/bot/message/reply")
            .header("Content-Type", "application/json")
            .header(header::AUTHORIZATION, format!("Bearer {}", auth_token.0))
            .json(&message_request) // `.json()`を使ってリクエストボディを設定します。
            .send()
            .await?
            .text()
            .await?;

        let sent_messages: SentMessagesResponse = serde_json::from_str(&body).map_err(|_| {
            anyhow!(GatewayError::FailedConvertResponse(
                body.to_string(),
                "SentMessagesResponse".to_string()
            ))
        })?;

        let new_messages = message_request.into_messages(sender, sent_messages);
        Ok(new_messages)
    }
    async fn send_line_push_messages(
        &self,
        auth_token: LineAuthToken,
        sender: Option<NewSendSender>,
        message_request: PushSendMessageRequest,
    ) -> anyhow::Result<NewSendMessages> {
        let body = self
            .client
            .post("https://api.line.me/v2/bot/message/push")
            .header("Content-Type", "application/json")
            .header(header::AUTHORIZATION, format!("Bearer {}", auth_token.0))
            .header("X-Line-Retry-Key", message_request.retry_key.clone())
            .json(&message_request) // `.json()`を使ってリクエストボディを設定します。
            .send()
            .await?
            .text()
            .await?;

        let sent_messages: SentMessagesResponse = serde_json::from_str(&body).map_err(|_| {
            anyhow!(GatewayError::FailedConvertResponse(
                body.to_string(),
                "SentMessagesResponse".to_string()
            ))
        })?;

        let new_messages = message_request.into_messages(sender, sent_messages);
        Ok(new_messages)
    }
}
