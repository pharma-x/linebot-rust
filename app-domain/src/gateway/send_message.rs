use crate::model::{
    message::{
        event::NewEvent,
        send_message::{NewSendMessages, NewSendSender},
    },
    user_auth::UserAuthData,
};
use async_trait::async_trait;

#[mockall::automock]
#[async_trait]
pub trait SendMessageGateway {
    async fn send_messages(
        &self,
        user_auth_data: UserAuthData,
        sender: Option<NewSendSender>,
        event: NewEvent,
    ) -> anyhow::Result<Vec<NewSendMessages>>;
}
