pub mod event_type;
pub mod line_user;
pub mod line_user_auth;
pub mod primary_user_id;
pub mod talk_room;
pub mod user;
pub mod user_auth;
pub mod user_event;
pub mod user_message;

use anyhow::anyhow;
use derive_new::new;
use std::marker::PhantomData;
use ulid::Ulid;

#[derive(new, Debug, Clone, Copy)]
pub struct Id<T> {
    pub value: Ulid,
    _marker: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn gen() -> Id<T> {
        Id::new(Ulid::new())
    }
}

impl<T> TryFrom<String> for Id<T> {
    type Error = anyhow::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ulid::from_string(&value)
            .map(|id| Self::new(id))
            .map_err(|err| anyhow!("{:?}", err))
    }
}
