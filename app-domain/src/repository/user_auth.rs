use crate::model::{
    line_user::LineUserProfile,
    user::UserProfile,
    user_auth::{LineUserAuthData, UserAuthData},
};
use async_trait::async_trait;

#[async_trait]
pub trait UserAuthRepository {
    async fn get_user_profile(&self, source: UserAuthData) -> anyhow::Result<UserProfile>;

    async fn get_line_user_profile(
        &self,
        source: LineUserAuthData,
    ) -> anyhow::Result<LineUserProfile>;
}
