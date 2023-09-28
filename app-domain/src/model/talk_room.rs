use chrono::{DateTime, Local, TimeZone};
use derive_new::new;

use super::{
    event::{Event, Message, NewMessage},
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
    pub latest_message: LatestMessage,
    pub latest_messaged_at: DateTime<Local>,
    pub sort_time: DateTime<Local>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(new, Clone)]
pub struct NewTalkRoom {
    pub id: Id<TalkRoom>,
    pub primary_user_id: PrimaryUserId,
    pub display_name: String,
    pub rsvp: bool,
    pub pinned: bool,
    pub follow: bool,
    pub latest_message: NewLatestMessage,
    pub latest_messaged_at: DateTime<Local>,
    pub sort_time: DateTime<Local>,
    pub created_at: DateTime<Local>,
}

#[derive(new, Clone)]
pub struct UpdateTalkRoom {
    pub id: Id<TalkRoom>,
    pub primary_user_id: PrimaryUserId,
    pub display_name: String,
    pub rsvp: bool,
    pub pinned: bool,
    pub follow: bool,
    pub latest_message: UpdateLatestMessage,
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

impl From<User> for NewTalkRoom {
    fn from(s: User) -> Self {
        let primary_user_id = s.id;
        let user_profile = s.user_profile;
        let display_name = match user_profile {
            UserProfile::Line(p) => p.display_name,
        };
        let local_now = Local::now();
        NewTalkRoom {
            id: Id::<TalkRoom>::gen(),
            primary_user_id,
            display_name,
            rsvp: false,
            pinned: false,
            follow: true,
            latest_message: NewLatestMessage::Follow,
            latest_messaged_at: local_now,
            sort_time: local_now,
            created_at: local_now,
        }
    }
}

impl From<(TalkRoom, Event)> for UpdateTalkRoom {
    fn from(s: (TalkRoom, Event)) -> Self {
        let talk_room = s.0;
        let event = s.1;
        match event {
            Event::Follow(e) => {
                let latest_messaged_at = Local.timestamp_opt(e.timestamp, 0).unwrap();
                Self {
                    id: talk_room.id,
                    primary_user_id: talk_room.primary_user_id,
                    display_name: talk_room.display_name,
                    rsvp: talk_room.rsvp,
                    pinned: talk_room.pinned,
                    follow: true,
                    latest_message: UpdateLatestMessage::Follow,
                    latest_messaged_at,
                    sort_time: talk_room.sort_time,
                    created_at: talk_room.created_at,
                    updated_at: latest_messaged_at,
                }
            }
            Event::Unfollow(e) => {
                let latest_messaged_at = Local.timestamp_opt(e.timestamp, 0).unwrap();
                Self {
                    id: talk_room.id,
                    primary_user_id: talk_room.primary_user_id,
                    display_name: talk_room.display_name,
                    rsvp: talk_room.rsvp,
                    pinned: talk_room.pinned,
                    follow: false,
                    latest_message: UpdateLatestMessage::Unfollow,
                    latest_messaged_at,
                    sort_time: talk_room.sort_time,
                    created_at: talk_room.created_at,
                    updated_at: latest_messaged_at,
                }
            }
            Event::Postback(e) => {
                let latest_messaged_at = Local.timestamp_opt(e.timestamp, 0).unwrap();
                Self {
                    id: talk_room.id,
                    primary_user_id: talk_room.primary_user_id,
                    display_name: talk_room.display_name,
                    rsvp: talk_room.rsvp,
                    pinned: talk_room.pinned,
                    follow: talk_room.follow,
                    latest_message: UpdateLatestMessage::Postback,
                    latest_messaged_at,
                    sort_time: talk_room.sort_time,
                    created_at: talk_room.created_at,
                    updated_at: latest_messaged_at,
                }
            }
            Event::VideoPlayComplete(e) => {
                let latest_messaged_at = Local.timestamp_opt(e.timestamp, 0).unwrap();
                Self {
                    id: talk_room.id,
                    primary_user_id: talk_room.primary_user_id,
                    display_name: talk_room.display_name,
                    rsvp: talk_room.rsvp,
                    pinned: talk_room.pinned,
                    follow: talk_room.follow,
                    latest_message: UpdateLatestMessage::VideoPlayComplete,
                    latest_messaged_at,
                    sort_time: talk_room.sort_time,
                    created_at: talk_room.created_at,
                    updated_at: latest_messaged_at,
                }
            }
            Event::Message(e) => {
                let latest_messaged_at = Local.timestamp_opt(e.timestamp, 0).unwrap();
                Self {
                    id: talk_room.id,
                    primary_user_id: talk_room.primary_user_id,
                    display_name: talk_room.display_name,
                    rsvp: true,
                    pinned: talk_room.pinned,
                    follow: talk_room.follow,
                    latest_message: UpdateLatestMessage::Message(e.message),
                    latest_messaged_at,
                    sort_time: talk_room.sort_time,
                    created_at: talk_room.created_at,
                    updated_at: latest_messaged_at,
                }
            }
        }
    }
}
