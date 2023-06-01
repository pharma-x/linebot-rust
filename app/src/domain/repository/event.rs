use crate::domain::model::{event::Event, talk_room::TalkRoom};
use async_trait::async_trait;

#[async_trait]
pub trait EventRepository {
    async fn create_event(&self, source: Event, talk_room: TalkRoom) -> anyhow::Result<()>;
}
