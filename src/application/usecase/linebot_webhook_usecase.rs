use crate::adapter::module::RepositoriesModuleExt;
use crate::application::model::line_user_auth::CreateLineUserAuth;
use crate::application::model::user_event::CreateUserEvent;
use crate::domain::repository::{
    line_user::LineUserRepository, line_user_auth::LineUserAuthRepository,
};

use anyhow::Ok;
use derive_new::new;
use std::sync::Arc;

#[derive(new)]
pub struct LinebotWebhookUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> LinebotWebhookUseCase<R> {
    pub async fn create_user(&self, source: CreateUserEvent) -> anyhow::Result<()> {
        // todo すでにUserが作られていたら、userやtalk roomを作らない処理を書く
        let user_profile = self
            .repositories
            .line_user_auth_repository()
            .get_user_profile(source.create_line_user_auth.try_into()?)
            .await?;

        let line_user = self
            .repositories
            .line_user_repository()
            .create_user(user_profile)
            .await?;

        let talk_room = self
            .repositories
            .talk_room_repository()
            .create_talk_room(line_user)
            .await?;

        // todo factoryの実装
        let message = self
            .repositories
            .message_repository()
            .create_messages(source.create_user_event.try_into()?)
            .await?;

        Ok(())
    }
}
