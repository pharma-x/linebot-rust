use chrono::{DateTime, Local};
use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct PrimaryUserTable {
    pub id: String,
    pub created_at: DateTime<Local>,
}
