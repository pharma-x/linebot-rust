use crate::domain::model::{primary_user_id::PrimaryUserId, talk_room::TalkRoom};
use async_trait::async_trait;

#[async_trait]
pub trait TalkRoomRepository {
    async fn create_talk_room(&self, source: PrimaryUserId) -> anyhow::Result<TalkRoom>;
}
