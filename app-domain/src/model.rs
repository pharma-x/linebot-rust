pub mod event;
pub mod line_user;
pub mod primary_user_id;
pub mod talk_room;
pub mod user;
pub mod user_auth;

use anyhow::anyhow;
use derive_new::new;
use std::marker::PhantomData;
use uuid::Uuid;

#[derive(new, Debug, Clone, Copy)]
pub struct Id<T> {
    pub value: Uuid,
    _marker: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn gen() -> Id<T> {
        Id::new(Uuid::new_v4())
    }
}

impl<T> TryFrom<String> for Id<T> {
    type Error = anyhow::Error;
    fn try_from(value: String) -> anyhow::Result<Self> {
        Uuid::parse_str(&value)
            .map(|id| Self::new(id))
            .map_err(|err| anyhow!("{:?}", err))
    }
}
