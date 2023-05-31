use super::persistance::{firestore::Firestore, mysql::Db};
use derive_new::new;
use reqwest::Client;
use std::marker::PhantomData;

pub mod event;
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
