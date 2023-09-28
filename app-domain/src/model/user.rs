use crate::model::line_user::LineUserProfile;
use derive_new::new;

use super::primary_user_id::PrimaryUserId;

#[derive(new, Clone)]
pub struct User {
    pub id: PrimaryUserId,
    pub user_profile: UserProfile,
}

#[derive(new, Clone)]
pub enum UserProfile {
    Line(LineUserProfile),
}
