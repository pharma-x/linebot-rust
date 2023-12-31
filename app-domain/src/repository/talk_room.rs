use crate::model::{
    primary_user_id::PrimaryUserId,
    talk_room::{NewTalkRoom, TalkRoom},
};
use async_trait::async_trait;

#[mockall::automock]
#[async_trait]
pub trait TalkRoomRepository {
    async fn get_talk_room(&self, source: PrimaryUserId) -> anyhow::Result<TalkRoom>;
    async fn create_talk_room(&self, source: NewTalkRoom) -> anyhow::Result<TalkRoom>;
    async fn create_messages(&self, source: NewTalkRoom) -> anyhow::Result<TalkRoom>;
}
