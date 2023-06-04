use crate::domain::model::user_auth::{AuthToken, AuthUserId, LineUserAuthData, UserAuthData};
use derive_new::new;

#[derive(new, Clone)]
pub struct CreateLineUserAuth {
    pub user_id: String,
}

impl TryFrom<CreateLineUserAuth> for UserAuthData {
    type Error = anyhow::Error;
    fn try_from(c: CreateLineUserAuth) -> anyhow::Result<Self> {
        // todo: 環境変数からlineのaccess tokenを取得する
        let auth_token = "token".to_string();
        Ok(UserAuthData::Line(LineUserAuthData {
            auth_id: AuthUserId::Line(c.user_id),
            token: AuthToken::Line(auth_token),
        }))
    }
}
