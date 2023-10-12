use std::env;
use std::sync::Arc;

use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

#[derive(Clone)]
pub struct Db(Arc<Pool<MySql>>);

impl Db {
    pub async fn new() -> Db {
        let pool = MySqlPoolOptions::new()
            .max_connections(8)
            .connect(&env::var("DATABASE_URL").unwrap_or_else(|_| {
                let mysql_user =
                    env::var("MYSQL_USER").unwrap_or_else(|_| panic!("MYSQL_USER is not set"));
                let mysql_password = env::var("MYSQL_PASSWORD")
                    .unwrap_or_else(|_| panic!("MYSQL_PASSWORD is not set"));
                let mysql_host =
                    env::var("MYSQL_HOST").unwrap_or_else(|_| panic!("MYSQL_HOST is not set"));
                let mysql_port =
                    env::var("MYSQL_PORT").unwrap_or_else(|_| panic!("MYSQL_PORT is not set"));
                let mysql_db =
                    env::var("MYSQL_DB").unwrap_or_else(|_| panic!("MYSQL_DB is not set"));
                format!(
                    "mysql://{}:{}@{}:{}/{}",
                    mysql_user, mysql_password, mysql_host, mysql_port, mysql_db
                )
            }))
            .await
            .unwrap_or_else(|_| {
                panic!("Cannot connect to the database. Please check your configuration.")
            });
        Db(Arc::new(pool))
    }

    pub fn pool(&self) -> &Arc<Pool<MySql>> {
        &self.0
    }
}
