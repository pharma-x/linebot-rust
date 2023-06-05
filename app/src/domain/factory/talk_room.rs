use crate::domain::model::{
    event::NewEvent,
    talk_room::{TalkRoom, UpdateTalkRoom},
};

pub trait TalkRoomFactory {
    fn create_update_talk_room_event(
        &self,
        talk_room: TalkRoom,
        new_event: NewEvent,
    ) -> UpdateTalkRoom;
}
