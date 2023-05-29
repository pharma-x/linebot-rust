use crate::domain::{model::event::Event, repository::event::EventRepository};

use super::FirestoreRepositoryImpl;
use async_trait::async_trait;

#[async_trait]
impl EventRepository for FirestoreRepositoryImpl<Event> {
    async fn create_event(&self, source: Event) -> anyhow::Result<()> {
        todo!()
    }
}
