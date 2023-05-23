use serde::Deserialize;

use crate::domain::model::line_user_auth::LineUserProfile;

#[derive(Deserialize)]
pub struct ResponseLineAuth {
    pub user_id: String,
    pub display_name: String,
    pub language: Option<String>,
    pub picture_url: Option<String>,
    pub status_message: Option<String>,
}

// 保存するにはidを生成する必要があるので、NewMarketKindに変換する
impl TryFrom<ResponseLineAuth> for LineUserProfile {
    type Error = anyhow::Error;
    fn try_from(s: ResponseLineAuth) -> anyhow::Result<Self> {
        Ok(LineUserProfile {
            user_id: s.user_id.into(),
            display_name: s.display_name,
            picture_url: s.picture_url.unwrap_or("".to_string()),
        })
    }
}
