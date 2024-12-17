use std::env;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub async fn create_pool() -> Pool<Postgres> {
    
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable must be set");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create PgPool")
}
