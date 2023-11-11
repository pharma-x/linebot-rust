use derive_new::new;
use reqwest::Client;
use std::marker::PhantomData;
use thiserror::Error;

pub mod send_message;
pub mod user_auth;

#[derive(new)]
pub struct HttpClientRepositoryImpl<T> {
    pub client: Client,
    _marker: PhantomData<T>,
}

#[derive(Debug, Error)]
pub enum GatewayError {
    #[error("Failed to convert response {0} to {1}")]
    FailedConvertResponse(String, String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
