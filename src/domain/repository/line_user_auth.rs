use crate::domain::model::{line_user_auth::LineUserAuthData, user::UserProfile};
use async_trait::async_trait;

#[async_trait]
pub trait LineUserAuthRepository {
    async fn get_user_profile(&self, source: LineUserAuthData) -> anyhow::Result<UserProfile>;
}
