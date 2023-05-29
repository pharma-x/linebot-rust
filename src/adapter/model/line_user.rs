use crate::domain::model::{
    line_user::{LineUser, LineUserProfile},
    primary_user_id::PrimaryUserId,
    user_auth::AuthUserId,
};
use chrono::{DateTime, Local};
use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct LineUserTable {
    pub primary_user_id: String,
    pub line_id: String,
    pub display_name: String,
    pub picture_url: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl TryFrom<LineUserTable> for LineUser {
    type Error = anyhow::Error;
    fn try_from(l: LineUserTable) -> Result<Self, Self::Error> {
        Ok(LineUser {
            id: PrimaryUserId::new(l.primary_user_id),
            user_profile: LineUserProfile::new(
                AuthUserId::new(l.line_id),
                l.display_name,
                l.picture_url,
            ),
        })
    }
}
