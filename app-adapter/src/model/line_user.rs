use chrono::{DateTime, Local};
use domain::model::{
    line_user::LineUserProfile,
    primary_user_id::PrimaryUserId,
    user::{User, UserProfile},
    user_auth::LineId,
};
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

impl TryFrom<LineUserTable> for User {
    type Error = anyhow::Error;
    fn try_from(l: LineUserTable) -> Result<Self, Self::Error> {
        Ok(User {
            id: PrimaryUserId::new(l.primary_user_id),
            user_profile: UserProfile::Line(LineUserProfile {
                auth_id: LineId::new(l.line_id),
                display_name: l.display_name,
                picture_url: l.picture_url,
            }),
        })
    }
}
