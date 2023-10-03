use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct PrimaryUserTable {
    pub id: String,
}
