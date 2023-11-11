use crate::model::event::CreateUserEvent;
use adapter::{module::AdaptersModuleExt, repository::RepositoryError};
use derive_new::new;
use domain::{
    gateway::{send_message::SendMessageGateway, user_auth::UserAuthGateway},
    model::{
        message::event::NewEvent,
        user_auth::{AuthUserId, LineId, LineUserAuthData, UserAuthData},
    },
    repository::{talk_room::TalkRoomRepository, user::UserRepository},
};
use futures::future;
use std::sync::Arc;

#[derive(new)]
pub struct LinebotWebhookUseCase<R: AdaptersModuleExt> {
    pub adapters: Arc<R>,
}

impl<R: AdaptersModuleExt> LinebotWebhookUseCase<R> {
    pub async fn create_follow_event(&self, source: CreateUserEvent) -> anyhow::Result<()> {
        /*
         * userを取得、なければ作成する
         */
        let create_line_user_auth = source.clone().create_line_user_auth;
        let res_user = self
            .adapters
            .user_repository()
            .get_user(AuthUserId::Line(LineId::from(
                create_line_user_auth.clone(),
            )))
            .await;

        let line_user_auth_data = LineUserAuthData::try_from(create_line_user_auth)?;
        let user = match res_user {
            Ok(s) => s,
            Err(anyhow_err) => {
                if let Some(RepositoryError::NotFound(_, _)) =
                    anyhow_err.downcast_ref::<RepositoryError>()
                {
                    let user_profile = self
                        .adapters
                        .user_auth_gateway()
                        .get_user_profile(UserAuthData::Line(line_user_auth_data.clone()))
                        .await?;
                    self.adapters
                        .user_repository()
                        .create_user(user_profile)
                        .await?
                } else {
                    // anyhow_errがRepositoryErrorではない場合
                    return Err(anyhow_err);
                }
            }
        };

        /*
         * talk_roomを取得し、
         * あればtalk_roomをupdateし、talk_roomのサブコレクションにeventを追加する
         * なければtalk_roomを作成し、talk_roomのサブコレクションevent作成する
         */
        let new_event = NewEvent::from(source.create_event);
        let res_talk_room = self
            .adapters
            .talk_room_repository()
            .get_talk_room(user.clone().id)
            .await;
        let updated_talk_room = match res_talk_room {
            Ok(talk_room) => {
                // talk_roomをupdateし、talk_roomのサブコレクションにeventを追加する
                self.adapters
                    .talk_room_repository()
                    .create_event((talk_room, new_event.clone()).into())
                    .await?
            }
            Err(anyhow_err) => {
                if let Some(RepositoryError::NotFound(_, _)) =
                    anyhow_err.downcast_ref::<RepositoryError>()
                {
                    self.adapters
                        .talk_room_repository()
                        .create_talk_room((user, new_event.clone()).into())
                        .await?
                } else {
                    return Err(anyhow_err);
                }
            }
        };

        // TODO: ここでメッセージを送る
        let new_sent_messages_vec = self
            .adapters
            .send_message_gateway()
            .send_messages(UserAuthData::Line(line_user_auth_data), None, new_event)
            .await?;

        // talk_roomをupdateし、talk_roomのサブコレクションにeventを追加する
        let works: Vec<_> = new_sent_messages_vec
            .into_iter()
            .map(|new_sent_messages| {
                self.adapters
                    .talk_room_repository()
                    .create_event((updated_talk_room.clone(), new_sent_messages.clone()).into())
            })
            .collect();

        future::try_join_all(works).await?;

        Ok(())
    }
}
