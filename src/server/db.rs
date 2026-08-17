use leptos::logging::log;
use leptos::prelude::{use_context, ServerFnError};
use sqlx::migrate::MigrateDatabase;
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::Postgres;

/// Crea il database se non esiste, apre il pool e applica le migration.
///
/// Va chiamata una volta sola allo startup, prima di iniziare a servire
/// richieste: se qualcosa va storto è meglio non partire affatto.
pub async fn init() -> Result<PgPool, sqlx::Error> {
    let url = std::env::var("DATABASE_URL").expect(
        "DATABASE_URL non impostata: definiscila in .env e avvia con `make dev`",
    );

    if !Postgres::database_exists(&url).await? {
        log!("Creating new DB");
        Postgres::create_database(&url).await?;
    }

    let pool = PgPoolOptions::new().max_connections(5).connect(&url).await?;

    // `migrate!` incorpora la cartella ./migrations nel binario a compile time,
    // quindi in produzione non serve avere i .sql sul disco.
    sqlx::migrate!("./migrations").run(&pool).await?;
    log!("Migrations applied");

    Ok(pool)
}

/// Recupera il pool dal contesto Leptos, dentro una server function.
pub fn pool() -> Result<PgPool, ServerFnError> {
    use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("Missing Postgres Pool in context"))
}
