use crate::domain::model::line_user_auth::{LineAuthToken, LineAuthUserId, LineUserAuthData};
use derive_new::new;

#[derive(new)]
pub struct CreateLineUserAuth {
    pub user_id: String,
}

impl TryFrom<CreateLineUserAuth> for LineUserAuthData {
    type Error = anyhow::Error;
    fn try_from(c: CreateLineUserAuth) -> anyhow::Result<Self> {
        // todo: 環境変数からlineのaccess tokenを取得する
        let auth_token = "token".to_string();
        Ok(LineUserAuthData::new(
            LineAuthUserId::new(c.user_id),
            LineAuthToken::new(auth_token),
        ))
    }
}
