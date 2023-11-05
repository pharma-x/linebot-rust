use std::any::type_name;
use std::sync::Arc;

use crate::model::{line_user::LineUserTable, primary_user::PrimaryUserTable};
use crate::repository::DatabaseRepositoryImpl;
use anyhow::{anyhow, Ok};
use async_trait::async_trait;
use domain::model::line_user::LineUserProfile;
use domain::model::user::{User, UserProfile};
use domain::model::user_auth::{AuthUserId, LineId};
use domain::model::Id;
use domain::repository::user::UserRepository;

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
        let pool = Arc::clone(self.pool.pool());
        let line_id = source.value().to_string();
        let line_user_row = sqlx::query_as::<_, LineUserTable>(
            r#"
                select primary_user_id, line_id, display_name, picture_url, created_at, updated_at from line_users
                where line_id = ?
                "#,
            )
        .bind(line_id.clone())
        .fetch_one(&*pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => anyhow!(RepositoryError::NotFound(type_name::<LineUserTable>().to_string(),line_id)),
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
        let pool = Arc::clone(self.pool.pool());
        let mut tx = pool.begin().await.expect("Unable to begin transaction");
        let primary_user_id = Id::<User>::gen().value.to_string();
        sqlx::query(
            r#"
            insert into primary_users (id, created_at)
            values (?, default);
            "#,
        )
        .bind(primary_user_id.clone())
        .execute(&mut *tx)
        .await
        .expect("Unable to insert a primary user");

        sqlx::query(
            r#"
            insert into line_users(primary_user_id, line_id, display_name, picture_url, created_at, updated_at)
            values (?, ?, ?, ?, default, default)
            "#,
        )
        .bind(primary_user_id)
        .bind(source.auth_id.value())
        .bind(source.display_name)
        .bind(source.picture_url)
        .execute(&mut *tx)
        .await
        .expect("Unable to insert a line user");

        tx.commit().await.expect("Unable to commit transaction");

        let line_user_row = sqlx::query_as::<_, LineUserTable>(
            r#"
            select * from line_users where line_id = LAST_INSERT_ID()
            "#,
        )
        .fetch_one(&*pool)
        .await
        .expect("Unable to fetch the line user");

        Ok(line_user_row.try_into()?)
    }
}
