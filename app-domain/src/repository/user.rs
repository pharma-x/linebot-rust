use crate::model::{
    line_user::LineUserProfile,
    user::{User, UserProfile},
    user_auth::{AuthUserId, LineId},
};
use async_trait::async_trait;

#[mockall::automock]
#[async_trait]
pub trait UserRepository {
    async fn get_user(&self, source: AuthUserId) -> anyhow::Result<User>;
    async fn get_line_user(&self, source: LineId) -> anyhow::Result<User>;

    async fn create_user(&self, source: UserProfile) -> anyhow::Result<User>;
    async fn create_line_user(&self, source: LineUserProfile) -> anyhow::Result<User>;
}

