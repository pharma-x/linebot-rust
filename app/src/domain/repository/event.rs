use crate::domain::model::{event::NewEvent, talk_room::UpdateTalkRoom};
use async_trait::async_trait;

#[async_trait]
pub trait EventRepository {
    async fn create_event(&self, update_talk_room: UpdateTalkRoom, new_event: NewEvent) -> anyhow::Result<()>;
}
