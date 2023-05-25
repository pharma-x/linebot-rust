use crate::domain::model::{
    line_user::{LineUser, LineUserProfile},
    user_auth::UserAuthData,
};
use async_trait::async_trait;

#[async_trait]
pub trait LineUserAuthRepository {
    async fn get_user_profile(
        &self,
        source: UserAuthData<LineUser>,
    ) -> anyhow::Result<LineUserProfile>;
}
