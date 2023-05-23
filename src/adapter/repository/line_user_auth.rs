use crate::adapter::model::line_user_auth::ResponseLineAuth;
use crate::domain::model::line_user_auth::LineUserAuthData;
use crate::domain::model::user::UserProfile;
use crate::domain::model::user_auth::{AuthToken, AuthUserId};
use crate::domain::repository::user_auth::UserAuthRepository;
use anyhow::Ok;
use async_trait::async_trait;
use derive_new::new;
use reqwest::{header, Client};

// 上位でLineUserAuthRepositoryの依存関係を注入する必要がある
#[derive(new)]
pub struct LineUserAuthRepository {}

#[async_trait]
impl UserAuthRepository for LineUserAuthRepository {
    type AuthData = LineUserAuthData;

    async fn get_user_profile(&self, source: LineUserAuthData) -> anyhow::Result<UserProfile> {
        let client = Client::new();
        let body = client
            .get(format!(
                "https://api.line.me/v2/bot/profile/{}",
                source.user_id.value()
            ))
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", source.token.value()),
            )
            .send()
            .await?
            .text()
            .await?;

        let res_line_auth: ResponseLineAuth = serde_json::from_str(&body).expect(&format!(
            "cannot convert ResponseLineAuth instance. body: {}",
            body
        ));

        Ok(UserProfile::Line(res_line_auth.try_into()?))
    }
}
