use chrono::{DateTime, Local};
use derive_new::new;

use super::{
    event::Message, line_user::LineUser, primary_user_id::PrimaryUserId, user::User,
    user::UserProfile, Id,
};

#[derive(new, Clone)]
pub struct TalkRoom {
    pub id: String,
    pub primary_user_id: PrimaryUserId,
    pub display_name: String,
    pub rsvp: bool,
    pub pinned: bool,
    pub follow: bool,
    pub latest_message: LatestMessage,
    pub latest_messaged_at: DateTime<Local>,
    pub sort_time: DateTime<Local>,
}

#[derive(Clone)]
pub enum LatestMessage {
    Follow,
    Unfollow,
    Postback,
    VideoPlayComplete,
    Message(Message),
}

impl From<LineUser> for TalkRoom {
    fn from(s: LineUser) -> Self {
        let primary_user_id = s.user_id();
        let user_profile = s.user_profile();
        let display_name = match user_profile {
            UserProfile::Line(p) => p.display_name,
        };

        TalkRoom {
            id: Id::<TalkRoom>::gen().value.to_string(),
            primary_user_id,
            display_name,
            rsvp: false,
            pinned: false,
            follow: true,
            latest_message: LatestMessage::Follow,
            latest_messaged_at: Local::now(),
            sort_time: Local::now(),
        }
    }
}
