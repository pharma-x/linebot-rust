use crate::{
    adapter::model::talk_room::{TalkRoomCardTable, TalkRoomTable},
    domain::{
        model::{
            primary_user_id::PrimaryUserId,
            talk_room::{NewTalkRoom, TalkRoom},
        },
        repository::talk_room::TalkRoomRepository,
    },
};

use super::{FirestoreRepositoryImpl, TALK_ROOM_CARD_COLLECTION_NAME, TALK_ROOM_COLLECTION_NAME};
use async_trait::async_trait;

#[async_trait]
impl TalkRoomRepository for FirestoreRepositoryImpl<TalkRoom> {
    async fn create_talk_room(&self, source: NewTalkRoom) -> anyhow::Result<TalkRoom> {
        let talk_room_table = TalkRoomTable::from(source.clone());
        let talk_room_card_table = TalkRoomCardTable::from(source.clone());
        let pool = self.pool.0.clone();
        pool.fluent()
            .insert()
            .into(TALK_ROOM_COLLECTION_NAME)
            .document_id(&talk_room_table.document_id)
            .object(&talk_room_table)
            .execute()
            .await?;
        pool.fluent()
            .insert()
            .into(TALK_ROOM_CARD_COLLECTION_NAME)
            .document_id(&talk_room_card_table.document_id)
            .object(&talk_room_card_table)
            .execute()
            .await?;

        let talk_room = TalkRoom {
            id: talk_room_table.document_id.try_into().unwrap(),
            primary_user_id: PrimaryUserId::new(talk_room_table.primary_user_id),
            display_name: talk_room_card_table.display_name,
            rsvp: talk_room_card_table.rsvp,
            pinned: talk_room_card_table.pinned,
            follow: talk_room_card_table.follow,
            latest_message: talk_room_card_table.latest_message.into(),
            latest_messaged_at: talk_room_card_table.latest_messaged_at,
            sort_time: talk_room_card_table.sort_time,
            created_at: talk_room_card_table.created_at,
            updated_at: talk_room_card_table.updated_at,
        };

        // todo table系からtalk_roomに変換する処理を記述する
        Ok(talk_room)
    }
    // todo talkRoomのfind処理も記述する
}
