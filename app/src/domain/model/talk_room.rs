use chrono::{DateTime, Local};
use derive_new::new;

use super::{
    event::{Message, NewMessage},
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
    Message(NewMessage),
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
