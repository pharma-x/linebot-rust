use crate::adapter::model::line_user_auth::ResponseLineAuth;
use crate::adapter::repository::HttpClientRepositoryImpl;
use crate::domain::model::line_user::LineUserProfile;
use crate::domain::model::user::UserProfile;
use crate::domain::model::user_auth::{LineUserAuthData, UserAuthData};
use crate::domain::repository::user_auth::UserAuthRepository;
use anyhow::Ok;
use async_trait::async_trait;
use reqwest::header;

#[async_trait]
impl UserAuthRepository for HttpClientRepositoryImpl<UserAuthData> {
    async fn get_user_profile(&self, source: UserAuthData) -> anyhow::Result<UserProfile> {
        let res = match source {
            UserAuthData::Line(d) => {
                UserProfile::Line(self.get_line_user_profile(d).await.unwrap())
            }
        };

        Ok(res)
    }

    async fn get_line_user_profile(
        &self,
        source: LineUserAuthData,
    ) -> anyhow::Result<LineUserProfile> {
        let body = &self
            .client
            .get(format!(
                "https://api.line.me/v2/bot/profile/{}",
                source.auth_id.value()
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

        Ok(res_line_auth.try_into()?)
    }
}
