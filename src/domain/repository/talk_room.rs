use crate::domain::model::{line_user::LineUser, talk_room::TalkRoom};
use async_trait::async_trait;

#[async_trait]
pub trait TalkRoomRepository {
    async fn create_talk_room(&self, source: LineUser) -> anyhow::Result<TalkRoom>;
}
