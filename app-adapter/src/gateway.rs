use derive_new::new;
use reqwest::Client;
use std::marker::PhantomData;

pub mod send_message;
pub mod user_auth;

#[derive(new)]
pub struct HttpClientRepositoryImpl<T> {
    pub client: Client,
    _marker: PhantomData<T>,
}
