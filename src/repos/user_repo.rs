use std::sync::Arc;
use sqlx::{Pool, Postgres, Error};
use crate::modals::user_modal::{CreateUser, User, UserWithCredits};

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

    pub async fn create_user(
        &self,
        user_data: CreateUser
    ) -> Result<User, Error> {

        let created_user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (
                email, display_name, photo_url, dob, gender, 
                is_whitelisted, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING 
                *
            "#
        )
        .bind(&user_data.email)
        .bind(&user_data.display_name)
        .bind(&user_data.photo_url)
        .bind(&user_data.dob)
        .bind(&user_data.gender)
        .bind(&user_data.is_whitelisted)
        .bind(user_data.created_at)
        .bind(user_data.updated_at)
        .fetch_one(self.pool.as_ref())
        .await?;

        Ok(created_user)
    }


    pub async fn get_user_by_id(
        &self, 
        user_id: &str
    ) -> Result<User, Error> {
        let user = sqlx::query_as::<_, User>(
            r#"
                SELECT * 
                FROM users 
                WHERE id = $1 and is_deleted = false
            "#)
            .bind(user_id)
            .fetch_one(self.pool.as_ref())
            .await?;
        Ok(user)
    }


    pub async fn get_user_with_credits(
        &self, 
        user_id: &str
    ) -> Result<UserWithCredits, Error> {
        
        let user_with_credits = sqlx::query_as::<_, UserWithCredits>(
            r#"
            SELECT 
                u.*,
                COALESCE(
                    json_agg(
                        json_build_object(
                            'credits', uc.credits,
                            'seconds_remaining', uc.seconds_remaining
                        )
                    ) FILTER (WHERE uc.user_id IS NOT NULL),
                    '[]'::json
                ) AS user_credits
            FROM users u
            LEFT JOIN user_credits uc 
                ON uc.user_id = u.id 
                AND uc.is_deleted = false
            WHERE 
                u.id = 'pAfQTEKhrwaNkLd04vSw6LHozGI3' 
                AND u.is_deleted = false
            GROUP BY u.id;

            "#,
        )
        .bind(user_id)
        .fetch_one(self.pool.as_ref())
        .await?;
    
        Ok(user_with_credits)
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

