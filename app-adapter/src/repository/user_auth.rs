use crate::model::line_user_auth::ResponseLineAuth;
use crate::repository::HttpClientRepositoryImpl;
use anyhow::Ok;
use async_trait::async_trait;
use domain::model::{
    line_user::LineUserProfile,
    user::UserProfile,
    user_auth::{LineUserAuthData, UserAuthData},
};
use domain::repository::user_auth::UserAuthRepository;
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
                format!("Bearer {}", source.auth_token()),
            )
            .send()
            .await?
            .text()
            .await?;

        let res_line_auth: ResponseLineAuth = serde_json::from_str(body)
            .unwrap_or_else(|_| panic!("cannot convert ResponseLineAuth instance. body: {}", body));

        Ok(res_line_auth.try_into()?)
    }
}
