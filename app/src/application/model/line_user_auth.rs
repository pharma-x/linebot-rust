use crate::domain::model::{
    line_user::LineUser,
    user_auth::{AuthToken, AuthUserId, UserAuthData},
};
use derive_new::new;

#[derive(new)]
pub struct CreateLineUserAuth {
    pub user_id: String,
}

impl TryFrom<CreateLineUserAuth> for UserAuthData<LineUser> {
    type Error = anyhow::Error;
    fn try_from(c: CreateLineUserAuth) -> anyhow::Result<Self> {
        // todo: 環境変数からlineのaccess tokenを取得する
        let auth_token = "token".to_string();
        Ok(UserAuthData {
            user_id: AuthUserId::new(c.user_id),
            token: AuthToken::new(auth_token),
        })
    }
}
