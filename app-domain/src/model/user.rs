use super::{primary_user_id::PrimaryUserId, user_auth::AuthUserId};
use crate::model::line_user::LineUserProfile;
use derive_new::new;

#[derive(new, Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: PrimaryUserId,
    pub user_profile: UserProfile,
}

#[derive(new, Debug, Clone, PartialEq, Eq)]
pub enum UserProfile {
    Line(LineUserProfile),
}

impl UserProfile {
    pub fn auth_id(&self) -> Option<&AuthUserId> {
        match self {
            UserProfile::Line(line) => Some(&line.auth_id),
        }
    }
}
