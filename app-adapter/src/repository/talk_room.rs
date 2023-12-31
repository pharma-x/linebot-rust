use anyhow::anyhow;
use async_trait::async_trait;
use std::sync::Arc;

use crate::model::message::event::EventTable;
use crate::model::message::send_message::SendMessageTable;
use crate::model::message::MessagesTable;
use crate::model::talk_room::{TalkRoomCardTable, TalkRoomDbTable, TalkRoomTable};
use crate::repository::{
    DbFirestoreRepositoryImpl, RepositoryError, MESSAGE_COLLECTION_NAME,
    TALK_ROOM_CARD_COLLECTION_NAME, TALK_ROOM_COLLECTION_NAME,
};
use domain::{
    model::{
        message::{Messages, NewMessages},
        primary_user_id::PrimaryUserId,
        talk_room::{NewTalkRoom, TalkRoom},
    },
    repository::talk_room::TalkRoomRepository,
};

#[async_trait]
impl TalkRoomRepository for DbFirestoreRepositoryImpl<TalkRoom> {
    async fn get_talk_room(&self, primary_user_id: PrimaryUserId) -> anyhow::Result<TalkRoom> {
        /*
         * DBのtalk_roomsテーブルからprimary_user_idを元にtalk_roomを取得する
         */
        let pool = Arc::clone(self.db.pool());
        let primary_user_id_str = primary_user_id.value().to_string();
        let talk_room_db_table = sqlx::query_as::<_, TalkRoomDbTable>(
            r#"
            select * from talk_rooms
            where primary_user_id = ?
            "#,
        )
        .bind(primary_user_id_str.clone())
        .fetch_one(&*pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                anyhow!(RepositoryError::NotFound(
                    "talk_rooms".to_string(),
                    primary_user_id_str.clone()
                ))
            }
            _ => anyhow!(RepositoryError::Unexpected(e.to_string())),
        })?;
        println!("talk_room_db_table: {:?}", talk_room_db_table);
        /*
         * FirestoreのtalkRoomsとtalkRoomCardsコレクションからdocument_idを元にtalk_roomとtalk_room_cardを取得する
         */
        let firestore = Arc::clone(&self.firestore.0);
        let document_id = talk_room_db_table.document_id;
        let talk_room_table: TalkRoomTable = firestore
            .fluent()
            .select()
            .by_id_in(TALK_ROOM_COLLECTION_NAME)
            .obj()
            .one(&document_id)
            .await?
            .ok_or(RepositoryError::NotFound(
                TALK_ROOM_COLLECTION_NAME.to_string(),
                document_id.clone(),
            ))?;
        println!("talk_room_table: {:?}", talk_room_table);

        let talk_room_card_table: TalkRoomCardTable = firestore
            .fluent()
            .select()
            .by_id_in(TALK_ROOM_CARD_COLLECTION_NAME)
            .obj()
            .one(&document_id)
            .await?
            .ok_or(RepositoryError::NotFound(
                TALK_ROOM_CARD_COLLECTION_NAME.to_string(),
                document_id.clone(),
            ))?;
        println!("talk_room_card_table: {:?}", talk_room_card_table);

        let message_document_id = talk_room_card_table.latest_message.document_id();
        let messages_table: MessagesTable = firestore
            .fluent()
            .select()
            .by_id_in(MESSAGE_COLLECTION_NAME)
            .parent(&firestore.parent_path(TALK_ROOM_COLLECTION_NAME, &document_id)?)
            .obj()
            .one(&message_document_id)
            .await?
            .ok_or(RepositoryError::NotFound(
                MESSAGE_COLLECTION_NAME.to_string(),
                message_document_id.to_string(),
            ))?;
        println!("messages_table: {:?}", messages_table.clone());
        let latest_messages = messages_table.into_messages(message_document_id);

        Ok(TalkRoom::new(
            document_id.try_into()?,
            primary_user_id,
            talk_room_card_table.display_name,
            talk_room_card_table.rsvp,
            talk_room_card_table.pinned,
            talk_room_card_table.follow,
            latest_messages,
            talk_room_card_table.latest_messaged_at,
            talk_room_card_table.sort_time,
            talk_room_card_table.created_at,
            talk_room_card_table.updated_at,
        ))
    }

    async fn create_talk_room(&self, source: NewTalkRoom) -> anyhow::Result<TalkRoom> {
        let db = Arc::clone(self.db.pool());
        let document_id = source.id.value.to_string();
        // firestoreの書き込みが失敗したときにもDBへの書き込みも失敗するようにする
        let mut tx = db.begin().await.expect("Unable to begin transaction");
        sqlx::query(
            r#"
            insert into talk_rooms(document_id, primary_user_id, created_at)
            values (?, ?, default)
            "#,
        )
        .bind(source.id.value.to_string())
        .bind(source.primary_user_id.value())
        .execute(&mut *tx)
        .await
        .expect("Unable to insert a talk rooms");

        let talk_room_table = TalkRoomTable::from(source.clone());
        println!("talk_room_table: {:?}", talk_room_table);
        let talk_room_card_table = TalkRoomCardTable::from(source.clone());
        println!("talk_room_card_table: {:?}", talk_room_card_table);
        let firestore = Arc::clone(&self.firestore.0);
        firestore
            .fluent()
            .insert()
            .into(TALK_ROOM_COLLECTION_NAME)
            .document_id(&document_id)
            .object(&talk_room_table)
            .execute()
            .await
            .map_err(|e| {
                println!("firestore insert error: {}", e);
                anyhow!(RepositoryError::CouldNotInsert(
                    TALK_ROOM_COLLECTION_NAME.to_string(),
                    "document_id".to_string(),
                    document_id.clone(),
                ))
            })?;
        firestore
            .fluent()
            .insert()
            .into(TALK_ROOM_CARD_COLLECTION_NAME)
            .document_id(&document_id)
            .object(&talk_room_card_table)
            .execute()
            .await
            .map_err(|e| {
                println!("firestore insert error: {}", e);
                anyhow!(RepositoryError::CouldNotInsert(
                    TALK_ROOM_CARD_COLLECTION_NAME.to_string(),
                    "document_id".to_string(),
                    document_id.clone(),
                ))
            })?;
        // トランザクションはスコープ外になると自動的にロールバックしてくれるので、firestoreでエラーが起きた場合もDBへの書き込みも削除される
        tx.commit().await.expect("Unable to commit the transaction");

        /*
         * イベントを作成する
         */
        let talk_room = self.create_messages(source.clone()).await?;

        Ok(talk_room)
    }

    /// talkRoomをupdateし、イベントを作成する
    ///
    /// # Arguments
    /// * `source` - 更新するtalkRoom。latest_messageには最新のイベントを入れる
    ///
    async fn create_messages(&self, source: NewTalkRoom) -> anyhow::Result<TalkRoom> {
        let firestore = Arc::clone(&self.firestore.0);
        let talk_room_document_id = source.id.value.to_string();
        let talk_room_card_table = TalkRoomCardTable::from(source.clone());
        firestore
            .fluent()
            .update()
            .in_col(TALK_ROOM_CARD_COLLECTION_NAME)
            .document_id(&talk_room_document_id)
            .object(&talk_room_card_table)
            .execute()
            .await?;
        let new_latest_messages = source.latest_messages;
        /*
         * イベントを作成する
         */
        let last_messages = self
            .insert_messages(&talk_room_document_id, &new_latest_messages)
            .await?;

        Ok(TalkRoom::new(
            talk_room_document_id.try_into()?,
            source.primary_user_id,
            talk_room_card_table.display_name,
            talk_room_card_table.rsvp,
            talk_room_card_table.pinned,
            talk_room_card_table.follow,
            last_messages,
            talk_room_card_table.latest_messaged_at,
            talk_room_card_table.sort_time,
            talk_room_card_table.created_at,
            talk_room_card_table.updated_at,
        ))
    }
}

impl DbFirestoreRepositoryImpl<TalkRoom> {
    async fn insert_messages(
        &self,
        talk_room_document_id: &String,
        new_latest_messages: &NewMessages,
    ) -> anyhow::Result<Messages> {
        let last_messages = match new_latest_messages {
            NewMessages::Event(e) => {
                let document_id = e.id().value.to_string();
                let event_table = EventTable::from(e.clone());
                self.insert_messages_table_to_firestore(
                    talk_room_document_id,
                    &document_id,
                    &MessagesTable::Event(event_table.clone()),
                )
                .await?;
                Messages::Event(event_table.into_event(&document_id))
            }
            NewMessages::SendMessages(m) => {
                let document_id = m.id.value.to_string();
                let send_message_table = SendMessageTable::from(m.clone());
                self.insert_messages_table_to_firestore(
                    talk_room_document_id,
                    &document_id,
                    &MessagesTable::SendMessage(send_message_table.clone()),
                )
                .await?;
                Messages::SendMessages(send_message_table.into_messages(&document_id))
            }
        };
        Ok(last_messages)
    }
    async fn insert_messages_table_to_firestore(
        &self,
        talk_room_document_id: &String,
        document_id: &String,
        messges_table: &MessagesTable,
    ) -> anyhow::Result<()> {
        let firestore = Arc::clone(&self.firestore.0);
        let parent_path =
            firestore.parent_path(TALK_ROOM_COLLECTION_NAME, talk_room_document_id)?;
        firestore
            .fluent()
            .insert()
            .into(MESSAGE_COLLECTION_NAME)
            .document_id(document_id)
            .parent(&parent_path)
            .object(messges_table)
            .execute()
            .await?;
        Ok(())
    }
}
