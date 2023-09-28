use crate::model::{line_user::LineUserTable, primary_user::PrimaryUserTable};
use crate::repository::DatabaseRepositoryImpl;
use domain::model::line_user::LineUserProfile;
use domain::model::user::{User, UserProfile};
use domain::model::user_auth::{AuthUserId, LineId};
use domain::repository::user::UserRepository;
use anyhow::{anyhow, Ok};
use async_trait::async_trait;

use super::RepositoryError;

#[async_trait]
impl UserRepository for DatabaseRepositoryImpl<User> {
    async fn get_user(&self, source: AuthUserId) -> anyhow::Result<User> {
        let res = match source {
            AuthUserId::Line(line_id) => self.get_line_user(line_id).await?,
        };

        Ok(res)
    }

    async fn get_line_user(&self, source: LineId) -> anyhow::Result<User> {
        let pool = self.pool.0.clone();
        let line_id = source.value().to_string();
        let line_user_row = sqlx::query_as::<_, LineUserTable>(
            r#"
select * from line_users(primary_user_id, line_id, display_name, picture_url, created_at, updated_at)
where id = $1
            "#,
        )
        .bind(line_id.clone())
        .fetch_one(&*pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => anyhow!(RepositoryError::NotAuthFound(line_id)),
            _ => anyhow!(RepositoryError::Unexpected(e.to_string())),
        })?;

        Ok(line_user_row.try_into()?)
    }

    async fn create_user(&self, source: UserProfile) -> anyhow::Result<User> {
        let res = match source {
            UserProfile::Line(line_user) => self.create_line_user(line_user).await.unwrap(),
        };

        Ok(res)
    }

    async fn create_line_user(&self, source: LineUserProfile) -> anyhow::Result<User> {
        let pool = self.pool.0.clone();
        let mut tx = pool.begin().await.expect("Unable to begin transaction");
        let primary_user_row = sqlx::query_as::<_, PrimaryUserTable>(
            r#"
insert into primary_users
values (default)
returning *"#,
        )
        .fetch_one(&mut tx)
        .await
        .expect("Unable to insert a primary user");

        let line_user_row = sqlx::query_as::<_, LineUserTable>(
            r#"
insert into line_users(primary_user_id, line_id, display_name, picture_url, created_at, updated_at)
values ($1, $2, $3, $4, default, default)
returning *"#,
        )
        .bind(primary_user_row.id)
        .bind(source.auth_id.value())
        .bind(source.display_name)
        .bind(source.picture_url)
        .fetch_one(&mut tx)
        .await
        .expect("Unable to insert a line user");

        tx.commit().await.expect("Unable to commit transaction");

        Ok(line_user_row.try_into()?)
    }
}
