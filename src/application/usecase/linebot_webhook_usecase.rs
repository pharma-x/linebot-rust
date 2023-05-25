use crate::adapter::module::RepositoriesModuleExt;
use crate::application::model::line_user_auth::CreateLineUserAuth;
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
    pub async fn create_user(&self, source: CreateLineUserAuth) -> anyhow::Result<()> {
        let user_profile = self
            .repositories
            .line_user_auth_repository()
            .get_user_profile(source.try_into()?)
            .await?;

        let line_user = self
            .repositories
            .line_user_repository()
            .create_user(user_profile)
            .await?;

        Ok(())
    }
}
