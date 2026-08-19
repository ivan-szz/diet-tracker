use chrono::{DateTime, NaiveDate, Utc};
use sqlx::PgPool;

use crate::{
    schema::entry::{
        CreateEntrySchema, DeleteEntrySchema, FindEntryByIdSchema, FindEntryByUserSchema,
        UpdateEntryNotesSchema,
    },
    server::error::ServerError,
};

pub struct Entry {
    pub id: i32,
    pub date: NaiveDate,
    pub user_id: i32,
    pub name: String,
    pub calories: i32,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Entry {
    pub async fn find_by_user(
        value: &FindEntryByUserSchema,
        pool: &PgPool,
    ) -> Result<Vec<Self>, ServerError> {
        let FindEntryByUserSchema { name } = value;

        let entries = sqlx::query_as!(
            Self,
            "SELECT entries.*
             FROM entries
             INNER JOIN users ON users.id = entries.user_id
             WHERE users.name = $1
             ORDER BY entries.date DESC, entries.id DESC",
            name,
        )
        .fetch_all(pool)
        .await?;

        Ok(entries)
    }

    pub async fn find_by_id(value: &FindEntryByIdSchema, pool: &PgPool) -> Result<Self, ServerError> {
        let FindEntryByIdSchema { id } = value;

        let entry = sqlx::query_as!(Self, "SELECT * FROM entries WHERE id = $1", id,)
            .fetch_one(pool)
            .await?;

        Ok(entry)
    }

    pub async fn create(value: &CreateEntrySchema, pool: &PgPool) -> Result<Self, ServerError> {
        let CreateEntrySchema {
            date,
            user_name,
            name,
            calories,
            notes,
        } = value;

        let entry = sqlx::query_as!(
            Self,
            "INSERT INTO entries (user_id, date, name, calories, notes)
             VALUES ((SELECT id FROM users WHERE name = $1), $2, $3, $4, $5)
             RETURNING *",
            user_name,
            date,
            name,
            calories,
            notes.as_deref(),
        )
        .fetch_one(pool)
        .await?;

        Ok(entry)
    }

    pub async fn update_notes(
        value: &UpdateEntryNotesSchema,
        pool: &PgPool,
    ) -> Result<Self, ServerError> {
        let UpdateEntryNotesSchema { id, notes } = value;

        let entry = sqlx::query_as!(
            Self,
            "UPDATE entries
             SET notes = $1
             WHERE id = $2
             RETURNING *",
            notes.as_deref(),
            id,
        )
        .fetch_one(pool)
        .await?;

        Ok(entry)
    }

    pub async fn delete_entry(value: &DeleteEntrySchema, pool: &PgPool) -> Result<(), ServerError> {
        let DeleteEntrySchema { id } = value;

        sqlx::query!("DELETE FROM entries WHERE id = $1", id,)
            .execute(pool)
            .await?;

        Ok(())
    }
}
