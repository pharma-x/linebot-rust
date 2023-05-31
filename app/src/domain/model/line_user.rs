use super::primary_user_id::PrimaryUserId;
use crate::domain::model::user::{User, UserProfile};
use crate::domain::model::user_auth::AuthUserId;
use derive_new::new;

#[derive(new, Clone)]
pub struct LineUser {
    id: PrimaryUserId,
    user_profile: LineUserProfile,
}

#[derive(new, Clone)]
pub struct LineUserProfile {
    // todo: 文字数にバリデーションつける
    pub auth_id: AuthUserId<LineUser>,
    pub display_name: String,
    pub picture_url: String,
}

impl User for LineUser {
    fn user_id(&self) -> PrimaryUserId {
        self.id.clone()
    }

    fn user_profile(&self) -> UserProfile {
        UserProfile::Line(self.user_profile.clone())
    }
}
