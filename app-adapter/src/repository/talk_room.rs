use crate::model::event::EventTable;
use crate::model::talk_room::{TalkRoomCardTable, TalkRoomDbTable, TalkRoomTable, TalkRoomWrapper};
use crate::repository::{
    DbFirestoreRepositoryImpl, RepositoryError, EVENT_COLLECTION_NAME,
    TALK_ROOM_CARD_COLLECTION_NAME, TALK_ROOM_COLLECTION_NAME,
};
use async_trait::async_trait;
use domain::{
    model::{
        primary_user_id::PrimaryUserId,
        talk_room::{NewTalkRoom, TalkRoom},
    },
    repository::talk_room::TalkRoomRepository,
};
use firestore::*;
use futures::StreamExt;
use std::sync::Arc;

#[async_trait]
impl TalkRoomRepository for DbFirestoreRepositoryImpl<TalkRoom> {
    async fn get_talk_room(&self, primary_user_id: PrimaryUserId) -> anyhow::Result<TalkRoom> {
        let primary_user_id_str = primary_user_id.value().to_string();
        let firestore = Arc::clone(&self.firestore.0);

        let talk_room_vec = firestore
            .fluent()
            .select()
            .fields(paths!(TalkRoomTable::{document_id, primary_user_id, created_at}))
            .from(TALK_ROOM_COLLECTION_NAME)
            .filter(|q| {
                q.for_all([q
                    .field(path!(TalkRoomTable::primary_user_id))
                    .eq(primary_user_id_str.clone())])
            })
            .obj()
            .stream_query()
            .await?
            .collect::<Vec<TalkRoomTable>>()
            .await;

        let talk_room_table = talk_room_vec
            .first()
            .ok_or(RepositoryError::NotFound(primary_user_id_str.clone()))?;
        let talk_room_document_id = &talk_room_table.document_id;
        let talk_room_card_table: TalkRoomCardTable = firestore
            .fluent()
            .select()
            .by_id_in(TALK_ROOM_CARD_COLLECTION_NAME)
            .obj()
            .one(talk_room_document_id)
            .await?
            .ok_or(RepositoryError::NotFound(primary_user_id_str))?;

        let event_document_id = talk_room_card_table.latest_message.document_id();
        let event_table: EventTable = firestore
            .fluent()
            .select()
            .by_id_in(EVENT_COLLECTION_NAME)
            .obj()
            .one(event_document_id)
            .await?
            .ok_or(RepositoryError::NotFound(event_document_id.to_string()))?;

        Ok(TalkRoomWrapper::from((talk_room_table.clone(), talk_room_card_table, event_table)).0)
    }

    async fn create_talk_room(&self, source: NewTalkRoom) -> anyhow::Result<TalkRoom> {
        let db = Arc::clone(self.db.pool());
        // firestoreの書き込みが失敗したときにもDBへの書き込みも
        let mut tx = db.begin().await.expect("Unable to begin transaction");
        // todo talk_roomsテーブルに紐づけを保管する
        let _talk_room_db_table = sqlx::query_as::<_, TalkRoomDbTable>(
            r#"
insert into talk_rooms(talk_room_id, primary_user_id, created_at)
values ($1, $2, default)
returning *"#,
        )
        .bind(source.id.value.to_string())
        .bind(source.primary_user_id.value())
        .fetch_one(&mut *tx)
        .await
        .expect("Unable to insert a primary user");

        let talk_room_table = TalkRoomTable::from(source.clone());
        let talk_room_card_table = TalkRoomCardTable::from(source.clone());
        let firestore = Arc::clone(&self.firestore.0);
        firestore
            .fluent()
            .insert()
            .into(TALK_ROOM_COLLECTION_NAME)
            .document_id(&talk_room_table.document_id)
            .object(&talk_room_table)
            .execute()
            .await?;
        firestore
            .fluent()
            .insert()
            .into(TALK_ROOM_CARD_COLLECTION_NAME)
            .document_id(&talk_room_card_table.document_id)
            .object(&talk_room_card_table)
            .execute()
            .await?;

        // トランザクションはスコープ外になると自動的にロールバックしてくれるので、firestoreでエラーが起きた場合もDBへの書き込みも削除される
        tx.commit().await.expect("Unable to commit the transaction");

        let event_document_id = talk_room_card_table.latest_message.document_id();
        let event_table: EventTable = firestore
            .fluent()
            .select()
            .by_id_in(EVENT_COLLECTION_NAME)
            .obj()
            .one(event_document_id)
            .await?
            .ok_or(RepositoryError::NotFound(event_document_id.to_string()))?;

        Ok(TalkRoomWrapper::from((talk_room_table.clone(), talk_room_card_table, event_table)).0)
    }

    /// talkRoomをupdateし、イベントを作成する
    ///
    /// # Arguments
    ///
    /// * `source` - 更新するtalkRoom。latest_messageには最新のイベントを入れる
    ///
    async fn create_event(&self, source: NewTalkRoom) -> anyhow::Result<()> {
        let firestore = Arc::clone(&self.firestore.0);
        let talk_room_id = source.id.value.to_string();
        let talk_room_card_table = TalkRoomCardTable::from(source.clone());
        firestore
            .fluent()
            .update()
            .fields(paths!(TalkRoomCardTable::{latest_message, latest_messaged_at, sort_time}))
            .in_col(TALK_ROOM_CARD_COLLECTION_NAME)
            .document_id(&talk_room_id)
            .object(&talk_room_card_table)
            .execute()
            .await?;
        /*
         * イベントを作成する
         */
        let parent_path = firestore.parent_path(TALK_ROOM_COLLECTION_NAME, &talk_room_id)?;
        let new_event = source.latest_message;
        let event_table = EventTable::from(new_event);
        firestore
            .fluent()
            .insert()
            .into(EVENT_COLLECTION_NAME)
            .document_id(event_table.document_id())
            .parent(&parent_path)
            .object(&event_table)
            .execute()
            .await?;
        Ok(())
    }
}
