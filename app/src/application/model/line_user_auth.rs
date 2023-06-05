use crate::domain::model::user_auth::{
    AuthToken, AuthUserId, LineAuthToken, LineId, LineUserAuthData, UserAuthData,
};
use derive_new::new;

#[derive(new, Clone)]
pub struct CreateLineUserAuth {
    pub user_id: String,
}

impl From<CreateLineUserAuth> for AuthUserId {
    fn from(c: CreateLineUserAuth) -> AuthUserId {
        AuthUserId::Line(LineId::new(c.user_id))
    }
}

impl TryFrom<CreateLineUserAuth> for UserAuthData {
    type Error = anyhow::Error;
    fn try_from(c: CreateLineUserAuth) -> anyhow::Result<Self> {
        // todo: 環境変数からlineのaccess tokenを取得する
        let auth_token = "token".to_string();
        Ok(UserAuthData::Line(LineUserAuthData {
            auth_id: AuthUserId::Line(LineId::new(c.user_id)),
            auth_token: AuthToken::Line(LineAuthToken::new(auth_token)),
        }))
    }
}
