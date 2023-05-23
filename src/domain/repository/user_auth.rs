use async_trait::async_trait;

use crate::domain::model::user::UserProfile;
use crate::domain::model::user_auth::UserAuthData;

#[async_trait]
pub trait UserAuthRepository {
    type AuthData: UserAuthData;

    async fn get_user_profile(&self, source: Self::AuthData) -> anyhow::Result<UserProfile>;
}
