use derive_new::new;
use std::marker::PhantomData;

pub mod event;
pub mod talk_room;

#[derive(new)]
pub struct FactoryImpl<T> {
    _marker: PhantomData<T>,
}
