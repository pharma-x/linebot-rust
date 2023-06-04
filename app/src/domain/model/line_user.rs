use crate::domain::model::user_auth::AuthUserId;
use derive_new::new;

#[derive(new, Clone)]
pub struct LineUserProfile {
    // todo: 文字数にバリデーションつける
    pub auth_id: AuthUserId,
    pub display_name: String,
    pub picture_url: String,
}
