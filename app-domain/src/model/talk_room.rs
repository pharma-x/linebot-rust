use chrono::{DateTime, Local};
use derive_new::new;

use crate::model::{
    event::{Event, Message, NewEvent, NewMessage},
    primary_user_id::PrimaryUserId,
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
    pub latest_message: Event,
    pub latest_messaged_at: DateTime<Local>,
    pub sort_time: DateTime<Local>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
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
    pub latest_message: NewEvent,
    pub latest_messaged_at: DateTime<Local>,
    pub sort_time: DateTime<Local>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(Clone)]
pub enum LatestMessage {
    Follow,
    Unfollow,
    Postback,
    VideoPlayComplete,
    Message(Message),
}

#[derive(Clone)]
pub enum NewLatestMessage {
    Follow,
    Unfollow,
    Postback,
    VideoPlayComplete,
    Message(NewMessage),
}

#[derive(Clone)]
pub enum UpdateLatestMessage {
    Follow,
    Unfollow,
    Postback,
    VideoPlayComplete,
    Message(Message),
}

impl From<(User, NewEvent)> for NewTalkRoom {
    fn from(s: (User, NewEvent)) -> Self {
        let user = s.0;
        let primary_user_id = user.id;
        let display_name = user
            .user_profile
            .display_name()
            .unwrap_or_else(|| &"".to_string());
        let new_event = s.1;
        let event_created_at = new_event.created_at().clone();
        let follow = new_event.follow();
        NewTalkRoom::new(
            Id::gen(),
            primary_user_id,
            display_name.clone(),
            false,
            false,
            follow,
            new_event,
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
        let event_created_at = new_event.created_at().clone();
        let follow = new_event.follow();
        NewTalkRoom::new(
            talk_room.id,
            talk_room.primary_user_id,
            talk_room.display_name,
            talk_room.rsvp,
            talk_room.pinned,
            follow,
            new_event,
            event_created_at,
            talk_room.sort_time,
            talk_room.created_at,
            event_created_at,
        )
    }
}
