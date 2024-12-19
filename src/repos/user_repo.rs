use crate::add_field;
use crate::modals::user_modal::{CreateUser, UpdateUser, User, UserWithCredits};
use sqlx::{Error, Pool, Postgres};
use std::sync::Arc;

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

    pub async fn create_user(&self, user_data: CreateUser) -> Result<User, Error> {
        let created_user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (
                email, display_name, photo_url, dob, gender, 
                is_whitelisted, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING 
                *
            "#,
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

    pub async fn get_user_by_id(&self, user_id: &str) -> Result<User, Error> {
        let user = sqlx::query_as::<_, User>(
            r#"
                SELECT * 
                FROM users 
                WHERE id = $1 and is_deleted = false
            "#,
        )
        .bind(user_id)
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(user)
    }

    pub async fn get_user_with_credits(&self, user_id: &str) -> Result<UserWithCredits, Error> {
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
                u.id = $1
                AND u.is_deleted = false
            GROUP BY u.id;

            "#,
        )
        .bind(user_id)
        .fetch_one(self.pool.as_ref())
        .await?;

        Ok(user_with_credits)
    }

    pub async fn update_user(
        &self,
        user_id: &str,
        user_data: UpdateUser,
    ) -> Result<(), sqlx::Error> {
        let mut query = String::from("UPDATE users SET ");
        let mut query_params = Vec::new();
        let mut param_index = 1;

        if user_data.name.is_some() {
            add_field!(query, query_params, param_index, "name", user_data.name);
        }
        if user_data.email.is_some() {
            add_field!(query, query_params, param_index, "email", user_data.email);
        }
        if user_data.display_name.is_some() {
            add_field!(
                query,
                query_params,
                param_index,
                "display_name",
                user_data.display_name
            );
        }
        if user_data.first_name.is_some() {
            add_field!(
                query,
                query_params,
                param_index,
                "first_name",
                user_data.first_name
            );
        }
        if user_data.last_name.is_some() {
            add_field!(
                query,
                query_params,
                param_index,
                "last_name",
                user_data.last_name
            );
        }
        if user_data.photo_url.is_some() {
            add_field!(
                query,
                query_params,
                param_index,
                "photo_url",
                user_data.photo_url
            );
        }
        if user_data.dob.is_some() {
            add_field!(
                query,
                query_params,
                param_index,
                "dob",
                user_data.dob.map(|v| v.to_string())
            );
        }
        if user_data.gender.is_some() {
            add_field!(
                query,
                query_params,
                param_index,
                "gender",
                user_data.gender.map(|v| format!("{:?}", v))
            );
        }
        if user_data.is_whitelisted.is_some() {
            add_field!(
                query,
                query_params,
                param_index,
                "is_whitelisted",
                user_data.is_whitelisted.map(|v| v.to_string())
            );
        }
        if user_data.created_at.is_some() {
            add_field!(
                query,
                query_params,
                param_index,
                "created_at",
                user_data.created_at.map(|v| v.to_string())
            );
        }
        if user_data.updated_at.is_some() {
            add_field!(
                query,
                query_params,
                param_index,
                "updated_at",
                user_data.updated_at.map(|v| v.to_string())
            );
        }
        if query_params.is_empty() {
            return Ok(());
        }

        query.push_str(&format!(" WHERE id = ${}", param_index));
        query_params.push(user_id.to_string());

        let mut sqlx_query = sqlx::query(&query);
        for param in query_params {
            sqlx_query = sqlx_query.bind(param);
        }

        println!("Query: {}", query);

        let result = sqlx_query.execute(self.pool.as_ref()).await?;
        println!("Rows affected: {}", result.rows_affected());

        Ok(())
    }
    pub async fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let query = format!("SELECT * FROM {}", self.table_name);
        let users = sqlx::query_as::<_, User>(&query)
            .fetch_all(self.pool.as_ref())
            .await?;
        Ok(users)
    }

    pub async fn delete_user(&self, user_id: &str) -> Result<(), Error> {
        let query = format!("DELETE FROM {} WHERE id = $1", self.table_name);
        let result = sqlx::query(&query)
            .bind(user_id)
            .execute(self.pool.as_ref())
            .await?;

        if result.rows_affected() > 0 {
            Ok({})
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }
}
