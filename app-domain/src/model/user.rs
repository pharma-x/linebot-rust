use crate::model::{
    line_user::LineUserProfile, primary_user_id::PrimaryUserId, user_auth::AuthUserId,
};
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
    pub fn auth_id(&self) -> Option<AuthUserId> {
        match self {
            UserProfile::Line(user_profile) => Some(AuthUserId::Line(user_profile.auth_id.clone())),
        }
    }
    pub fn display_name(&self) -> Option<&String> {
        match self {
            UserProfile::Line(user_profile) => Some(&user_profile.display_name),
        }
    }
}
