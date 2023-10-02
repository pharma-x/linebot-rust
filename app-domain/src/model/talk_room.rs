use chrono::{DateTime, Local};
use derive_new::new;

use crate::model::{
    event::{Event, Message, NewEvent, NewMessage},
    primary_user_id::PrimaryUserId,
    user::User,
    user::UserProfile,
    Id,
};

#[derive(new, Clone)]
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
#[derive(Clone)]
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
        let user_profile = user.user_profile;
        let display_name = match user_profile {
            UserProfile::Line(p) => p.display_name,
        };
        let new_event = s.1;
        // todo eventのtimestampを使う
        let local_now = Local::now();
        NewTalkRoom {
            id: Id::gen(),
            primary_user_id,
            display_name,
            rsvp: false,
            pinned: false,
            follow: true,
            latest_message: new_event,
            latest_messaged_at: local_now,
            sort_time: local_now,
            created_at: local_now,
            updated_at: local_now,
        }
    }
}

impl From<(TalkRoom, NewEvent)> for NewTalkRoom {
    fn from(s: (TalkRoom, NewEvent)) -> Self {
        let talk_room = s.0;
        let event = s.1;
        match event {
            NewEvent::Follow(e) => {
                let event_created_at = e.created_at;
                Self {
                    id: talk_room.id,
                    primary_user_id: talk_room.primary_user_id,
                    display_name: talk_room.display_name,
                    rsvp: talk_room.rsvp,
                    pinned: talk_room.pinned,
                    follow: true,
                    latest_message: NewEvent::Follow(e),
                    latest_messaged_at: event_created_at,
                    sort_time: talk_room.sort_time,
                    created_at: talk_room.created_at,
                    updated_at: event_created_at,
                }
            }
            NewEvent::Unfollow(e) => {
                let event_created_at = e.created_at;
                Self {
                    id: talk_room.id,
                    primary_user_id: talk_room.primary_user_id,
                    display_name: talk_room.display_name,
                    rsvp: talk_room.rsvp,
                    pinned: talk_room.pinned,
                    follow: false,
                    latest_message: NewEvent::Unfollow(e),
                    latest_messaged_at: event_created_at,
                    sort_time: talk_room.sort_time,
                    created_at: talk_room.created_at,
                    updated_at: event_created_at,
                }
            }
            NewEvent::Postback(e) => {
                let event_created_at = e.created_at;
                Self {
                    id: talk_room.id,
                    primary_user_id: talk_room.primary_user_id,
                    display_name: talk_room.display_name,
                    rsvp: talk_room.rsvp,
                    pinned: talk_room.pinned,
                    follow: talk_room.follow,
                    latest_message: NewEvent::Postback(e),
                    latest_messaged_at: event_created_at,
                    sort_time: talk_room.sort_time,
                    created_at: talk_room.created_at,
                    updated_at: event_created_at,
                }
            }
            NewEvent::VideoPlayComplete(e) => {
                let event_created_at = e.created_at;
                Self {
                    id: talk_room.id,
                    primary_user_id: talk_room.primary_user_id,
                    display_name: talk_room.display_name,
                    rsvp: talk_room.rsvp,
                    pinned: talk_room.pinned,
                    follow: talk_room.follow,
                    latest_message: NewEvent::VideoPlayComplete(e),
                    latest_messaged_at: event_created_at,
                    sort_time: talk_room.sort_time,
                    created_at: talk_room.created_at,
                    updated_at: event_created_at,
                }
            }
            NewEvent::Message(e) => {
                let event_created_at = e.created_at;
                Self {
                    id: talk_room.id,
                    primary_user_id: talk_room.primary_user_id,
                    display_name: talk_room.display_name,
                    rsvp: true,
                    pinned: talk_room.pinned,
                    follow: talk_room.follow,
                    latest_message: NewEvent::Message(e),
                    latest_messaged_at: event_created_at,
                    sort_time: talk_room.sort_time,
                    created_at: talk_room.created_at,
                    updated_at: event_created_at,
                }
            }
        }
    }
}
