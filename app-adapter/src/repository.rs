use crate::persistance::{firestore::Firestore, mysql::Db};
use derive_new::new;
use std::marker::PhantomData;
use thiserror::Error;

pub mod talk_room;
pub mod user;

const TALK_ROOM_COLLECTION_NAME: &str = "talkRooms";
const TALK_ROOM_CARD_COLLECTION_NAME: &str = "talkRoomCards";
const MESSAGE_COLLECTION_NAME: &str = "messages";

#[derive(new)]
pub struct DatabaseRepositoryImpl<T> {
    pub pool: Db,
    _marker: PhantomData<T>,
}

#[derive(new)]
pub struct FirestoreRepositoryImpl<T> {
    pub pool: Firestore,
    _marker: PhantomData<T>,
}

#[derive(new)]
pub struct DbFirestoreRepositoryImpl<T> {
    pub db: Db,
    pub firestore: Firestore,
    _marker: PhantomData<T>,
}

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Unexpected Error: {0}")]
    Unexpected(String),
    #[error("NotAuthFound, auth_id is {0}")]
    NotAuthFound(String),
    #[error("NotFound, table is {0}, id is {1}")]
    NotFound(String, String),
    #[error("CouldNotInsert, table is {0}, column {1} is {2}")]
    CouldNotInsert(String, String, String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
