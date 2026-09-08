use dioxus::fullstack::FullstackContext;
use dioxus::logger::tracing::info;
use dioxus::prelude::ServerFnError;
use sqlx::migrate::MigrateDatabase;
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::Postgres;
use std::sync::OnceLock;

static POOL: OnceLock<PgPool> = OnceLock::new();

pub async fn init() -> Result<PgPool, sqlx::Error> {
    if let Some(pool) = POOL.get() {
        return Ok(pool.clone());
    }

    let url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL non impostata: definiscila in .env");

    if cfg!(debug_assertions) && !Postgres::database_exists(&url).await? {
        info!("Creating new DB");
        Postgres::create_database(&url).await?;
    }

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;

    // `migrate!` embeds the ./migrations folder into the binary at compile time,
    // so the .sql files don't need to be on disk in production.
    sqlx::migrate!("./migrations").run(&pool).await?;
    info!("Migrations applied");

    Ok(POOL.get_or_init(|| pool).clone())
}

pub fn pool() -> Result<PgPool, ServerFnError> {
    FullstackContext::current()
        .and_then(|ctx| ctx.extension::<PgPool>())
        .ok_or_else(|| ServerFnError::new("Missing Postgres Pool in context"))
}
