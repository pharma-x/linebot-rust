use crate::model::event::{Event, NewEvent};
use async_trait::async_trait;

#[async_trait]
pub trait EventRepository {
    async fn create_event(
        &self,
        new_event: NewEvent,
    ) -> anyhow::Result<Event>;
}
