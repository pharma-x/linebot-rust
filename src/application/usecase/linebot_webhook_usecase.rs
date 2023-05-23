use crate::application::model::line_auth::CreateLineAuth;
use crate::domain::repository::user_auth::UserAuthRepository;
use derive_new::new;
use std::sync::Arc;

#[derive(new)]
pub struct LinebotWebhookUseCase<R: UserAuthRepository> {
    repositories: Arc<R>,
}

impl<R: UserAuthRepository> LinebotWebhookUseCase<R> {
    pub async fn create_user(&self, source: CreateLineAuth) -> anyhow::Result<()> {
        todo!()
    }
}
