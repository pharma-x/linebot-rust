use crate::domain::{
    model::{primary_user_id::PrimaryUserId, talk_room::TalkRoom},
    repository::talk_room::TalkRoomRepository,
};

use super::FirestoreRepositoryImpl;
use async_trait::async_trait;

#[async_trait]
impl TalkRoomRepository for FirestoreRepositoryImpl<TalkRoom> {
    async fn create_talk_room(&self, source: PrimaryUserId) -> anyhow::Result<TalkRoom> {
        todo!()
    }
}
