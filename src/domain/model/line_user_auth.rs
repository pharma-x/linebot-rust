use crate::domain::model::user_auth::{AuthToken, AuthUserId};
use derive_new::new;

use super::user_auth::UserAuthData;

#[derive(new)]
pub struct LineAuthUserId {
    pub value: String,
}

impl AuthUserId for LineAuthUserId {
    fn value(&self) -> &String {
        &self.value
    }
}

#[derive(new)]
pub struct LineAuthToken {
    pub value: String,
}

impl AuthToken for LineAuthToken {
    fn value(&self) -> &String {
        &self.value
    }
}

#[derive(new)]
pub struct LineUserAuthData {
    pub user_id: LineAuthUserId,
    pub token: LineAuthToken,
}

impl UserAuthData for LineUserAuthData {
    type UserId = LineAuthUserId;
    type Token = LineAuthToken;

    fn user_id(&self) -> &Self::UserId {
        &self.user_id
    }
    fn token(&self) -> &Self::Token {
        &self.token
    }
}

#[derive(new)]
pub struct LineUserProfile {
    // todo: 文字数にバリデーションつける
    pub user_id: LineAuthUserId,
    pub display_name: String,
    pub profile_image_url: String,
}
