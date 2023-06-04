use crate::domain::model::talk_room::{NewTalkRoom, TalkRoom};
use async_trait::async_trait;

#[async_trait]
pub trait TalkRoomRepository {
    async fn create_talk_room(&self, source: NewTalkRoom) -> anyhow::Result<TalkRoom>;
}
