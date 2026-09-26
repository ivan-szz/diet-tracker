use crate::schema::day::{CreateDaySchema, FindDayByUserSchema};
use crate::schema::entry::{
    CreateEntrySchema, DeleteEntrySchema, EntryFiltersSchema, EntrySchema, UpdateEntryNotesSchema,
};
use crate::server::error::ServerError;
use crate::server::repo::day::Day;
use crate::server::repo::entry::Entry;
use crate::server::services::{day, non_blank, users};
use sqlx::PgPool;

pub async fn list(
    user_name: &str,
    filters: EntryFiltersSchema,
    pool: &PgPool,
) -> Result<Vec<EntrySchema>, ServerError> {
    users::find(user_name, pool).await?;
    let filters = EntryFiltersSchema {
        q: non_blank(filters.q),
        ..filters
    };
    let entries = Entry::find_by_user(user_name, &filters, pool).await?;

    Ok(entries.into_iter().map(EntrySchema::from).collect())
}

pub async fn create(
    payload: &CreateEntrySchema,
    pool: &PgPool,
) -> Result<EntrySchema, ServerError> {
    let existing_day = Day::find_one_by_user(
        &FindDayByUserSchema {
            user_name: payload.user_name.clone(),
            date: payload.date,
        },
        pool,
    )
    .await?;

    // The diary is built from `Day` rows, so an entry on a day that doesn't
    // exist yet would never show up.
    if existing_day.is_none() {
        day::create(
            &CreateDaySchema {
                date: payload.date,
                user_name: payload.user_name.clone(),
                weight_kg: None,
                target_calories: None,
                notes: None,
            },
            pool,
        )
        .await?;
    }

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
