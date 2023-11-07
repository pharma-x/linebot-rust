use chrono::{DateTime, Local};
use derive_new::new;

use crate::model::{
    event::{Event, NewEvent},
    primary_user_id::PrimaryUserId,
    send_message::{NewSendMessages, SendMessage},
    user::User,
    Id,
};

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct TalkRoom {
    pub id: Id<TalkRoom>,
    pub primary_user_id: PrimaryUserId,
    pub display_name: String,
    pub rsvp: bool,
    pub pinned: bool,
    pub follow: bool,
    pub latest_messages: LatestMessages,
    pub latest_messaged_at: DateTime<Local>,
    pub sort_time: DateTime<Local>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LatestMessages {
    Event(Event),
    SendMessages(Vec<SendMessage>),
}

// talkRoomのupdate時にも使う
#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct NewTalkRoom {
    pub id: Id<TalkRoom>,
    pub primary_user_id: PrimaryUserId,
    pub display_name: String,
    pub rsvp: bool,
    pub pinned: bool,
    pub follow: bool,
    pub latest_messages: NewLatestMessages,
    pub latest_messaged_at: DateTime<Local>,
    pub sort_time: DateTime<Local>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

// talkRoomのupdate時にも使う
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NewLatestMessages {
    Event(NewEvent),
    SendMessages(NewSendMessages),
}

impl From<(User, NewEvent)> for NewTalkRoom {
    fn from(s: (User, NewEvent)) -> Self {
        let user = s.0;
        let primary_user_id = user.id;
        let default_display_name = String::from("");
        let display_name = user
            .user_profile
            .display_name()
            .unwrap_or(&default_display_name);
        let new_event = s.1;
        let event_created_at = *new_event.created_at();
        let follow = new_event.follow();
        NewTalkRoom::new(
            Id::gen(),
            primary_user_id,
            display_name.clone(),
            false,
            false,
            follow,
            NewLatestMessages::Event(new_event),
            event_created_at,
            event_created_at,
            event_created_at,
            event_created_at,
        )
    }
}

impl From<(TalkRoom, NewEvent)> for NewTalkRoom {
    fn from(s: (TalkRoom, NewEvent)) -> Self {
        let talk_room = s.0;
        let new_event = s.1;
        let event_created_at = *new_event.created_at();
        let follow = new_event.follow();
        NewTalkRoom::new(
            talk_room.id,
            talk_room.primary_user_id,
            talk_room.display_name,
            talk_room.rsvp,
            talk_room.pinned,
            follow,
            NewLatestMessages::Event(new_event),
            event_created_at,
            talk_room.sort_time,
            talk_room.created_at,
            event_created_at,
        )
    }
}

impl From<(TalkRoom, NewSendMessages)> for NewTalkRoom {
    fn from(s: (TalkRoom, NewSendMessages)) -> Self {
        let talk_room = s.0;
        let new_send_messages = s.1;
        // send_messagesはすべてのsend_messageのcreated_atが同じ
        let send_messages_created_at = *new_send_messages.messages[0].created_at();
        NewTalkRoom::new(
            talk_room.id,
            talk_room.primary_user_id,
            talk_room.display_name,
            talk_room.rsvp,
            talk_room.pinned,
            talk_room.follow,
            NewLatestMessages::SendMessages(new_send_messages),
            send_messages_created_at,
            talk_room.sort_time,
            talk_room.created_at,
            send_messages_created_at,
        )
    }
}
