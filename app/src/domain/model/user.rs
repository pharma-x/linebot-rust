use super::primary_user_id::PrimaryUserId;
use crate::domain::model::line_user::LineUserProfile;

pub trait User {
    fn user_id(&self) -> PrimaryUserId;
    fn user_profile(&self) -> UserProfile;
}

pub enum UserProfile {
    Line(LineUserProfile),
}
