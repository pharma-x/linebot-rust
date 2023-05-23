use derive_new::new;
use crate::domain::model::line_user_auth::{LineAuthUserId, LineAuthToken, LineUserAuthData};

#[derive(new)]
pub struct CreateLineAuth {
    pub line_id: String,
}


impl TryFrom<CreateLineAuth> for LineUserAuthData {
    type Error = anyhow::Error;
    fn try_from(c: CreateLineAuth) -> anyhow::Result<Self> {
        // todo: 環境変数からlineのaccess tokenを取得する
        let auth_token = "token".to_string();
        Ok(LineUserAuthData::new(
            LineAuthUserId::new(c.line_id),
            LineAuthToken::new(auth_token),
        ))
    }
}
