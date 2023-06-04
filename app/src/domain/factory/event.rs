use crate::{
    application::model::event::CreateEvent,
    domain::model::{event::NewEvent, primary_user_id::PrimaryUserId},
};

pub trait EventFactory {
    fn create_new_event(
        &self,
        primary_user_id: PrimaryUserId,
        create_event: CreateEvent,
    ) -> NewEvent;
}
