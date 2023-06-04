use crate::{
    adapter::model::{
        event::EventTable,
        talk_room::{TalkRoomCardTable, TalkRoomTable},
    },
    domain::{
        model::{
            event::{Event, NewEvent},
            talk_room::UpdateTalkRoom,
        },
        repository::event::EventRepository,
    },
};

use super::{
    FirestoreRepositoryImpl, EVENT_COLLECTION_NAME, TALK_ROOM_CARD_COLLECTION_NAME,
    TALK_ROOM_COLLECTION_NAME,
};
use async_trait::async_trait;
use firestore::*;
use futures::stream::BoxStream;
use futures::StreamExt;

#[async_trait]
impl EventRepository for FirestoreRepositoryImpl<Event> {
    async fn create_event(
        &self,
        update_talk_room: UpdateTalkRoom,
        new_event: NewEvent,
    ) -> anyhow::Result<()> {
        let primary_user_id = new_event.clone().primary_user_id().value().to_string();

        let pool = self.pool.0.clone();

        let talk_room_stream: BoxStream<TalkRoomTable> = pool
            .fluent()
            .select()
            .fields(paths!(TalkRoomTable::{document_id, primary_user_id, created_at}))
            .from(TALK_ROOM_COLLECTION_NAME)
            .filter(|q| {
                q.for_all([q
                    .field(path!(TalkRoomTable::primary_user_id))
                    .eq(primary_user_id.clone())])
            })
            .obj()
            .stream_query()
            .await?;
        let talk_room_vec: Vec<TalkRoomTable> = talk_room_stream.collect().await;
        let talk_room = talk_room_vec.first().unwrap_or_else(|| {
            panic!(
                "Cannot find talk_room by primary_user_id: {}",
                primary_user_id
            )
        });
        let talk_room_id = talk_room.document_id.clone();
        let parent_path = pool.parent_path(TALK_ROOM_COLLECTION_NAME, &talk_room_id)?;

        let event_table = EventTable::from(new_event);
        pool.fluent()
            .insert()
            .into(EVENT_COLLECTION_NAME)
            .document_id(event_table.document_id())
            .parent(&parent_path)
            .object(&event_table)
            .execute()
            .await?;

        // todo retry処理を記述したい
        let mut transaction = pool.begin_transaction().await?;

        let talk_room_card_table = TalkRoomCardTable::from(update_talk_room);
        pool.fluent()
            .update()
            .fields(paths!(TalkRoomCardTable::{latest_message, latest_messaged_at, sort_time}))
            .in_col(TALK_ROOM_CARD_COLLECTION_NAME)
            .document_id(&talk_room_id)
            .object(&talk_room_card_table)
            .add_to_transaction(&mut transaction)?;

        transaction.commit().await?;

        Ok(())
    }
}
