use crate::{
    adapter::model::talk_room::{TalkRoomCardTable, TalkRoomTable},
    domain::{
        model::{line_user::LineUser, talk_room::TalkRoom},
        repository::talk_room::TalkRoomRepository,
    },
};

use super::{FirestoreRepositoryImpl, TALK_ROOM_CARD_COLLECTION_NAME, TALK_ROOM_COLLECTION_NAME};
use async_trait::async_trait;

#[async_trait]
impl TalkRoomRepository for FirestoreRepositoryImpl<TalkRoom> {
    async fn create_talk_room(&self, source: LineUser) -> anyhow::Result<TalkRoom> {
        let talk_room = TalkRoom::from(source);
        let talk_room_table = TalkRoomTable::from(talk_room.clone());
        let pool = self.pool.0.clone();
        pool.fluent()
            .insert()
            .into(TALK_ROOM_COLLECTION_NAME)
            .document_id(&talk_room_table.document_id)
            .object(&talk_room_table)
            .execute()
            .await?;
        let talk_room_card_table = TalkRoomCardTable::from(talk_room.clone());
        pool.fluent()
            .insert()
            .into(TALK_ROOM_CARD_COLLECTION_NAME)
            .document_id(&talk_room_card_table.document_id)
            .object(&talk_room_card_table)
            .execute()
            .await?;

        Ok(talk_room)
    }
    // todo talkRoomのfind処理も記述する
}
