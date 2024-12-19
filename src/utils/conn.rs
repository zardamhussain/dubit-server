use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

pub async fn create_pool() -> Pool<Postgres> {
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set");

    PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(std::time::Duration::from_secs(30))
        .connect(&database_url)
        .await
        .expect("Failed to create PgPool")
}
