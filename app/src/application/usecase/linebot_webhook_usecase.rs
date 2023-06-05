use crate::adapter::{
    module::FactoriesModuleExt, module::RepositoriesModuleExt, repository::RepositoryError,
};
use crate::application::model::event::CreateUserEvent;
use crate::domain::factory::{event::EventFactory, talk_room::TalkRoomFactory};
use crate::domain::repository::{
    event::EventRepository, talk_room::TalkRoomRepository, user::UserRepository,
    user_auth::UserAuthRepository,
};
use derive_new::new;
use std::sync::Arc;

#[derive(new)]
pub struct LinebotWebhookUseCase<R: RepositoriesModuleExt, F: FactoriesModuleExt> {
    repositories: Arc<R>,
    factories: Arc<F>,
}

impl<R: RepositoriesModuleExt, F: FactoriesModuleExt> LinebotWebhookUseCase<R, F> {
    pub async fn create_user(&self, source: CreateUserEvent) -> anyhow::Result<()> {
        let create_line_user_auth = source.clone().create_line_user_auth;

        let res = self
            .repositories
            .user_repository()
            .get_user(create_line_user_auth.clone().into())
            .await;
        let user = match res {
            Ok(s) => s,
            Err(anyhow_err) => {
                if let Some(repository_err) = anyhow_err.downcast_ref::<RepositoryError>() {
                    match repository_err {
                        RepositoryError::NotAuthFound(_) => {
                            let user_profile = self
                                .repositories
                                .user_auth_repository()
                                .get_user_profile(create_line_user_auth.try_into()?)
                                .await?;
                            self.repositories
                                .user_repository()
                                .create_user(user_profile)
                                .await?
                        }
                        _ => return Err(anyhow_err),
                    }
                } else {
                    // anyhow_errはRepositoryErrorではない場合
                    return Err(anyhow_err);
                }
            }
        };

        // todo すでにtalk_roomが存在したら、createではなく、find_talk_roomを呼んでtalk_roomを返す
        let talk_room = self
            .repositories
            .talk_room_repository()
            .create_talk_room(user.clone().into())
            .await?;

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
