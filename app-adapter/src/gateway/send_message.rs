use anyhow::anyhow;
use async_trait::async_trait;
use reqwest::header;

use crate::{
    gateway::HttpClientRepositoryImpl,
    model::send_message::{BotSendMessageRequest, SentMessages},
    repository::RepositoryError,
};
use domain::{
    gateway::send_message::SendMessageGateway,
    model::{
        event::NewEvent,
        send_message::{NewSendMessages, SendMessage},
        user_auth::{AuthToken, UserAuthData},
    },
};

#[async_trait]
impl SendMessageGateway for HttpClientRepositoryImpl<SendMessage> {
    async fn send_messages(
        &self,
        user_auth_data: UserAuthData,
        event: NewEvent,
    ) -> anyhow::Result<NewSendMessages> {
        let messages = match user_auth_data {
            UserAuthData::Line(line_user_auth) => {
                println!("send_messages:{:?}", &event);
                let bot_message_request = BotSendMessageRequest::from_event(event);
                self.send_line_bot_messages(line_user_auth.auth_token, bot_message_request)
                    .await?
            }
        };

        Ok(messages)
    }
}

impl HttpClientRepositoryImpl<SendMessage> {
    async fn send_line_bot_messages(
        &self,
        auth_token: AuthToken,
        bot_message_request: BotSendMessageRequest,
    ) -> anyhow::Result<NewSendMessages> {
        let auth_token_str = auth_token.value();
        let body = self
            .client
            .post("https://api.line.me/v2/bot/message/reply")
            .header("Content-Type", "application/json")
            .header(header::AUTHORIZATION, format!("Bearer {}", auth_token_str))
            .json(&bot_message_request) // `.json()`を使ってリクエストボディを設定します。
            .send()
            .await?
            .text()
            .await?;

        println!("send_line_bot_messages body: {}", &body);
        // todo エラーを作成
        let sent_messages: SentMessages = serde_json::from_str(&body)
            .map_err(|e| anyhow!(RepositoryError::Unexpected(e.to_string())))?;

        let new_messages = bot_message_request.into_messages(sent_messages);
        Ok(new_messages)
    }
}
