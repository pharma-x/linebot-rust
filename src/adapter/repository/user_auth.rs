use crate::domain::model::user_auth::UserAuthData;
use derive_new::new;
use reqwest::Client;
use std::marker::PhantomData;

#[derive(new)]
pub struct UserAuthRepository<T: UserAuthData> {
    pub client: Client,
    _marker: PhantomData<T>,
}
