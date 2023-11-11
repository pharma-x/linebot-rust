use crate::model::user_auth::LineId;
use derive_new::new;

#[derive(new, Debug, Clone, PartialEq, Eq)]
pub struct LineUserProfile {
    // todo: 文字数にバリデーションつける
    pub auth_id: LineId,
    pub display_name: String,
    pub picture_url: String,
}
