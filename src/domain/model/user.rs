use crate::domain::model::line_user_auth::LineUserProfile;
use crate::domain::model::Id;

pub struct User {
    pub id: Id<User>,
    pub user_profile: UserProfile,
}

pub enum UserProfile {
    Line(LineUserProfile),
}
