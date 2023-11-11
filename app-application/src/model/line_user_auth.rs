use std::env;

use derive_new::new;
use domain::model::user_auth::{LineAuthToken, LineId, LineUserAuthData};

#[derive(new, Clone, Debug)]
pub struct CreateLineUserAuth {
    pub user_id: String,
}

impl From<CreateLineUserAuth> for LineId {
    fn from(c: CreateLineUserAuth) -> Self {
        LineId::new(c.user_id)
    }
}

impl TryFrom<CreateLineUserAuth> for LineUserAuthData {
    type Error = anyhow::Error;
    fn try_from(c: CreateLineUserAuth) -> anyhow::Result<Self> {
        let auth_token = env::var("LINE_ACCESS_TOKEN")
            .unwrap_or_else(|_| panic!("LINE_ACCESS_TOKEN is not set"));
        Ok(LineUserAuthData {
            auth_id: LineId::new(c.user_id),
            auth_token: LineAuthToken::new(auth_token),
        })
    }
}
