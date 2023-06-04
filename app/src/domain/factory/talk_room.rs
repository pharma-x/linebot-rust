use crate::domain::model::{talk_room::{TalkRoom, UpdateTalkRoom}, event::NewEvent};

pub trait TalkRoomFactory {
    fn create_update_talk_room_event(
        &self,
        talk_room: TalkRoom,
        new_event: NewEvent,
    ) -> UpdateTalkRoom;
}
