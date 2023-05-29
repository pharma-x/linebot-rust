use super::persistance::{mysql::Db, firestore::Firestore};
use derive_new::new;
use reqwest::Client;
use std::marker::PhantomData;

pub mod line_user;
pub mod line_user_auth;
pub mod talk_room;
pub mod user_auth;

#[derive(new)]
pub struct HttpClientRepositoryImpl<T> {
    pub client: Client,
    _marker: PhantomData<T>,
}

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
