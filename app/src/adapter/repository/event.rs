use crate::{
    adapter::model::{event::EventTable, talk_room::TalkRoomCardTable},
    domain::{
        model::{event::Event, talk_room::TalkRoom},
        repository::event::EventRepository,
    },
};

use super::{
    FirestoreRepositoryImpl, EVENT_COLLECTION_NAME, TALK_ROOM_CARD_COLLECTION_NAME,
    TALK_ROOM_COLLECTION_NAME,
};
use async_trait::async_trait;

#[async_trait]
impl EventRepository for FirestoreRepositoryImpl<Event> {
    async fn create_event(&self, source: Event, talk_room: TalkRoom) -> anyhow::Result<()> {
        let event_table = EventTable::from(source);

        let pool = self.pool.0.clone();
        let parent_path =
            pool.parent_path(TALK_ROOM_COLLECTION_NAME, &event_table.talk_room_id())?;

        // todo retry処理を記述したい
        let mut transaction = pool.begin_transaction().await?;

        pool.fluent()
            .insert()
            .into(EVENT_COLLECTION_NAME)
            .document_id(&event_table.document_id())
            .parent(&parent_path)
            .object(&event_table)
            .execute()
            .await?;

        let talk_room_card_table = TalkRoomCardTable::from(talk_room.clone());
        let latest_message = talk_room_card_table.latest_message;
        pool
            .fluent()
            .update()
            .fields(firestore::paths!(TalkRoomCardTable::{latest_message, latest_messaged_at, sort_time}))
            .in_col(TALK_ROOM_CARD_COLLECTION_NAME)
            .document_id(&event_table.talk_room_id())
            .object(&TalkRoomCardTable {
                latest_message,
                latest_messaged_at: event_table.created_at(),
                sort_time: event_table.created_at(),
                ..talk_room_card_table.clone()
            })
            .execute()
            .await?;

        transaction.commit().await?;

        Ok(())
    }
}
