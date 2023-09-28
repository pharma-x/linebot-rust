use crate::model::{
    primary_user_id::PrimaryUserId,
    talk_room::{NewTalkRoom, TalkRoom},
};
use async_trait::async_trait;

#[async_trait]
pub trait TalkRoomRepository {
    async fn get_talk_room(&self, source: PrimaryUserId) -> anyhow::Result<TalkRoom>;
    async fn create_talk_room(&self, source: NewTalkRoom) -> anyhow::Result<TalkRoom>;
    async fn update_talk_room(&self, source: TalkRoom) -> anyhow::Result<()>;
}
