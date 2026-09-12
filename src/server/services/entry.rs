use crate::schema::entry::{
    CreateEntrySchema, DeleteEntrySchema, EntrySchema, FindEntriesByUserSchema,
    UpdateEntryNotesSchema,
};
use crate::server::error::ServerError;
use crate::server::repo::entry::Entry;
use sqlx::PgPool;

pub async fn list(user_name: &str, pool: &PgPool) -> Result<Vec<EntrySchema>, ServerError> {
    let entries = Entry::find_by_user(
        &FindEntriesByUserSchema {
            name: user_name.to_string(),
        },
        pool,
    )
    .await?;

    Ok(entries.into_iter().map(EntrySchema::from).collect())
}

pub async fn create(
    payload: &CreateEntrySchema,
    pool: &PgPool,
) -> Result<EntrySchema, ServerError> {
    let entry = Entry::create(payload, pool).await?;
    Ok(entry.into())
}

pub async fn update_notes(
    payload: &UpdateEntryNotesSchema,
    pool: &PgPool,
) -> Result<EntrySchema, ServerError> {
    let entry = Entry::update_notes(payload, pool).await?;
    Ok(entry.into())
}

pub async fn delete(payload: &DeleteEntrySchema, pool: &PgPool) -> Result<(), ServerError> {
    Entry::delete_entry(payload, pool).await
}
