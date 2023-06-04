use crate::domain::model::{
    line_user::LineUserProfile,
    user::{User, UserProfile},
};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn create_user(&self, source: UserProfile) -> anyhow::Result<User>;
    async fn create_line_user(&self, source: LineUserProfile) -> anyhow::Result<User>;
}
