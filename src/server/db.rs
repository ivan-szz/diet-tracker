use dioxus::fullstack::FullstackContext;
use dioxus::logger::tracing::info;
use dioxus::prelude::ServerFnError;
use sqlx::migrate::MigrateDatabase;
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::Postgres;
use std::sync::OnceLock;

static POOL: OnceLock<PgPool> = OnceLock::new();

/// Creates the database if it does not exist, opens the pool and runs the migrations.
///
/// Must be called exactly once at startup, before serving any request: if
/// something goes wrong it is better not to start at all.
///
/// The result is memoized because `dx serve` rebuilds the router on every
/// hot-patch: without `OnceLock` each rebuild would open a brand new pool.
pub async fn init() -> Result<PgPool, sqlx::Error> {
    if let Some(pool) = POOL.get() {
        return Ok(pool.clone());
    }

    let url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL non impostata: definiscila in .env");

    if !Postgres::database_exists(&url).await? {
        info!("Creating new DB");
        Postgres::create_database(&url).await?;
    }

    let pool = PgPoolOptions::new().max_connections(5).connect(&url).await?;

    // `migrate!` embeds the ./migrations folder into the binary at compile time,
    // so the .sql files don't need to be on disk in production.
    sqlx::migrate!("./migrations").run(&pool).await?;
    info!("Migrations applied");

    Ok(POOL.get_or_init(|| pool).clone())
}

/// Retrieves the pool from inside a server function.
///
/// Dioxus has no separate "server context": the pool travels in the HTTP
/// request extensions (inserted in `main` via `Extension(pool)`).
/// `FullstackContext` exposes them both when the server function is called
/// over HTTP by the client and when it is called in-process during SSR.
pub fn pool() -> Result<PgPool, ServerFnError> {
    FullstackContext::current()
        .and_then(|ctx| ctx.extension::<PgPool>())
        .ok_or_else(|| ServerFnError::new("Missing Postgres Pool in context"))
}
