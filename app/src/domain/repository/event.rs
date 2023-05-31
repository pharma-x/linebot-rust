use crate::domain::model::event::Event;
use async_trait::async_trait;

#[async_trait]
pub trait EventRepository {
    async fn create_event(&self, source: Event) -> anyhow::Result<()>;
}
