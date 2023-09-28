use super::{
    DbFirestoreRepositoryImpl, RepositoryError, TALK_ROOM_CARD_COLLECTION_NAME,
    TALK_ROOM_COLLECTION_NAME,
};
use crate::model::talk_room::{
    TalkRoomCardTable, TalkRoomDbTable, TalkRoomTable
};
use domain::{
    model::{
        primary_user_id::PrimaryUserId,
        talk_room::{NewTalkRoom, TalkRoom},
    },
    repository::talk_room::TalkRoomRepository,
};
use async_trait::async_trait;
use firestore::*;
use futures::stream::BoxStream;
use futures::StreamExt;

#[async_trait]
impl TalkRoomRepository for DbFirestoreRepositoryImpl<TalkRoom> {
    async fn get_talk_room(&self, primary_user_id: PrimaryUserId) -> anyhow::Result<TalkRoom> {
        let primary_user_id = primary_user_id.value().to_string();
        let firestore = Arc::Clone(self.firestore.0);

        let talk_room_stream: BoxStream<TalkRoomTable> = firestore
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
        let talk_room_table = talk_room_vec
            .first()
            .ok_or(RepositoryError::NotFound(primary_user_id.clone()))?;

        let talk_room_card_table: TalkRoomCardTable = firestore
            .fluent()
            .select()
            .by_id_in(TALK_ROOM_CARD_COLLECTION_NAME)
            .obj()
            .one(&talk_room_table.document_id)
            .await?
            .ok_or(RepositoryError::NotFound(primary_user_id))?;

        Ok((talk_room_table, talk_room_card_table).into())
    }

    async fn create_talk_room(&self, source: NewTalkRoom) -> anyhow::Result<TalkRoom> {
        let db = self.db.0.clone();
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
        .fetch_one(&mut tx)
        .await
        .expect("Unable to insert a primary user");

        let talk_room_table = TalkRoomTable::from(source.clone());
        let talk_room_card_table = TalkRoomCardTable::from(source.clone());
        let firestore = Arc::Clone(self.firestore.0);
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

        Ok((talk_room_table, talk_room_card_table).into())
    }

    async fn update_talk_room(&self, source: UpdateTalkRoom) -> anyhow::Result<()> {
        let firestore = Arc::Clone(self.firestore.0);
        let talk_room_card_table = TalkRoomCardTable::from(source);
        firestore.fluent()
            .update()
            .fields(paths!(TalkRoomCardTable::{latest_message, latest_messaged_at, sort_time}))
            .in_col(TALK_ROOM_CARD_COLLECTION_NAME)
            .document_id(&talk_room_id)
            .object(&talk_room_card_table)
            .execute()
            .await?;

        Ok(())
    }
}
