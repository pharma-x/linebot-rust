use derive_new::new;
use domain::model::user_auth::{
    AuthToken, AuthUserId, LineAuthToken, LineId, LineUserAuthData, UserAuthData,
};

#[derive(new, Clone, Debug)]
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
        let auth_token = env::var("LINE_ACCESS_TOKEN")
            .unwrap_or_else(|_| panic!("LINE_ACCESS_TOKEN is not set"));
        Ok(UserAuthData::Line(LineUserAuthData {
            auth_id: AuthUserId::Line(LineId::new(c.user_id)),
            auth_token: AuthToken::Line(LineAuthToken::new(auth_token)),
        }))
    }
}
