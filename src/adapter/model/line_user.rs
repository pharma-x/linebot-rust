use crate::domain::model::{line_user::{LineUserProfile, LineUser}, user_auth::AuthUserId, primary_user_id::PrimaryUserId};
use chrono::{Local, NaiveDateTime, TimeZone};
use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct LineUserTable {
    pub primary_user_id: String,
    pub line_id: String,
    pub display_name: String,
    pub picture_url: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
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
            )}
        )
    }
}
