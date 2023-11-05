use serde::Deserialize;

use domain::model::{
    line_user::LineUserProfile,
    user_auth::{AuthUserId, LineId},
};

#[derive(Deserialize)]
pub struct ResponseLineAuth {
    #[serde(rename(deserialize = "userId"))]
    pub user_id: String,
    #[serde(rename(deserialize = "displayName"))]
    pub display_name: String,
    pub language: Option<String>,
    #[serde(rename(deserialize = "pictureUrl"))]
    pub picture_url: Option<String>,
    #[serde(rename(deserialize = "statusMessage"))]
    pub status_message: Option<String>,
}

// 保存するにはidを生成する必要があるので、NewMarketKindに変換する
impl TryFrom<ResponseLineAuth> for LineUserProfile {
    type Error = anyhow::Error;
    fn try_from(s: ResponseLineAuth) -> anyhow::Result<Self> {
        Ok(LineUserProfile {
            auth_id: AuthUserId::Line(LineId::new(s.user_id)),
            display_name: s.display_name,
            picture_url: s.picture_url.unwrap_or("".to_string()),
        })
    }
}
    