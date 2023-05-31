use crate::domain::model::line_user::{LineUser, LineUserProfile};
use async_trait::async_trait;

#[async_trait]
pub trait LineUserRepository {
    async fn create_user(&self, source: LineUserProfile) -> anyhow::Result<LineUser>;
}
