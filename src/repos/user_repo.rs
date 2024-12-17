use std::sync::Arc;
use sqlx::{Pool, Postgres, Error};
use crate::modals::user_modal::User;

#[derive(Debug)]
pub struct UserRepo {
    pool: Arc<Pool<Postgres>>,
    table_name: String,
}

impl UserRepo {
    pub fn new(pool: Arc<Pool<Postgres>>, table_name: Option<&str>) -> Self {
        Self {
            pool,
            table_name: table_name.unwrap_or("users").to_string(),
        }
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<User, Error> {
        let query = format!("SELECT * FROM {} WHERE email = $1", self.table_name);
        let user = sqlx::query_as::<_, User>(&query)
            .bind(email)
            .fetch_one(self.pool.as_ref())
            .await?;
        Ok(user)
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let query = format!("SELECT * FROM {}", self.table_name);
        let users = sqlx::query_as::<_, User>(&query)
            .fetch_all(self.pool.as_ref())
            .await?;
        Ok(users)
    }

    pub async fn delete_user(&self, user_id: i32) -> Result<u64, Error> {
        let query = format!("DELETE FROM {} WHERE id = $1", self.table_name);
        let result = sqlx::query(&query)
            .bind(user_id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(result.rows_affected())
    }
}

