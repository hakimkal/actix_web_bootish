use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Pool, Postgres};
use tracing::info;

pub async fn connect_db(database_url: &str) -> PgPool {
    info!("Connecting to database...");

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(database_url)
        .await
        .expect("Failed to create PostgreSQL pool");
    info!("Connected to PostgreSQL");

    // Run Migrations on Startup
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    info!("Migrations applied successfully!");
    pool
}
