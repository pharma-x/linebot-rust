use crate::adapter::module::RepositoriesModuleExt;
use crate::application::factory::event::EventFactory;
use crate::application::model::event::CreateUserEvent;
use crate::domain::repository::{
    event::EventRepository, line_user::LineUserRepository, line_user_auth::LineUserAuthRepository,
    talk_room::TalkRoomRepository,
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
        let user_profile = self
            .repositories
            .line_user_auth_repository()
            .get_user_profile(source.create_line_user_auth.try_into()?)
            .await?;

        // todo すでにUserが存在したら、createではなく、find_userを呼んでuserを返す
        let line_user = self
            .repositories
            .line_user_repository()
            .create_user(user_profile)
            .await?;

        // todo すでにtalk_roomが存在したら、createではなく、find_talk_roomを呼んでtalk_roomを返す
        let talk_room = self
            .repositories
            .talk_room_repository()
            .create_talk_room(line_user)
            .await?;

        let event = EventFactory::new().create_event(talk_room.clone(), source.create_user_event);
        self.repositories
            .event_repository()
            .create_event(event, talk_room.clone())
            .await?;

        Ok(())
    }
}
