use crate::adapter::module::{FactoriesModuleExt, RepositoriesModuleExt};
use crate::application::model::event::CreateUserEvent;
use crate::domain::factory::{event::EventFactory, talk_room::TalkRoomFactory};
use crate::domain::repository::{
    event::EventRepository, talk_room::TalkRoomRepository, user::UserRepository,
    user_auth::UserAuthRepository,
};

use anyhow::Ok;
use derive_new::new;
use std::sync::Arc;

#[derive(new)]
pub struct LinebotWebhookUseCase<R: RepositoriesModuleExt, F: FactoriesModuleExt> {
    repositories: Arc<R>,
    factories: Arc<F>,
}

impl<R: RepositoriesModuleExt, F: FactoriesModuleExt> LinebotWebhookUseCase<R, F> {
    pub async fn create_user(&self, source: CreateUserEvent) -> anyhow::Result<()> {
        let user_profile = self
            .repositories
            .user_auth_repository()
            .get_user_profile(source.clone().create_line_user_auth.try_into()?)
            .await?;

        // todo すでにUserが存在したら、createではなく、find_userを呼んでuserを返す
        let user = self
            .repositories
            .user_repository()
            .create_user(user_profile)
            .await?;

        // todo すでにtalk_roomが存在したら、createではなく、find_talk_roomを呼んでtalk_roomを返す
        let talk_room = self
            .repositories
            .talk_room_repository()
            .create_talk_room(user.clone().into())
            .await?;

        // todo create_eventではなく、create_user_eventを渡して、repositoryの中でuser＆talk_roomを取得する処理を記述する
        let new_event = self
            .factories
            .event_factory()
            .create_new_event(talk_room.clone().primary_user_id, source.create_event);
        let update_talk_room = self
            .factories
            .talk_room_factory()
            .create_update_talk_room_event(talk_room, new_event.clone());
        self.repositories
            .event_repository()
            .create_event(update_talk_room, new_event)
            .await?;

        Ok(())
    }
}
