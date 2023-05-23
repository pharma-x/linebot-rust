use crate::domain::model::line_user_auth::LineUserAuthData;
use crate::domain::model::user::UserProfile;
use crate::domain::repository::user_auth::UserAuthRepository;
use async_trait::async_trait;
use derive_new::new;

// 上位でLineUserAuthRepositoryの依存関係を注入する必要がある
#[derive(new)]
pub struct LineUserAuthRepository {}

#[async_trait]
impl UserAuthRepository for LineUserAuthRepository {
    type AuthData = LineUserAuthData;

    async fn get_user_profile(
        &self,
        source: LineUserAuthData,
    ) -> anyhow::Result<Option<UserProfile>> {
        todo!()
    }
}
