use crate::domain::{
    model::{line_user::LineUser, talk_room::TalkRoom},
    repository::talk_room::TalkRoomRepository,
};

use super::FirestoreRepositoryImpl;
use async_trait::async_trait;

#[async_trait]
impl TalkRoomRepository for FirestoreRepositoryImpl<TalkRoom> {
    async fn create_talk_room(&self, source: LineUser) -> anyhow::Result<TalkRoom> {
        todo!()
    }
}
