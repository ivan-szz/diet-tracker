use chrono::{DateTime, NaiveDate, Utc};
use sqlx::PgPool;

use crate::{
    schema::day::{
        CreateDaySchema, FindDayByUserSchema, FindDaysByUserSchema, UpdateDayNotesSchema,
        UpdateDayTargetCaloriesSchema, UpdateDayWeightSchema,
    },
    server::error::ServerError,
};

pub struct Day {
    pub id: i32,
    pub date: NaiveDate,
    pub user_id: i32,
    pub weight_kg: Option<f32>,
    pub target_calories: i32,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Day {
    pub async fn find_by_user(
        value: &FindDaysByUserSchema,
        pool: &PgPool,
    ) -> Result<Vec<Self>, ServerError> {
        let FindDaysByUserSchema { name } = value;

        let days = sqlx::query_as!(
            Self,
            "SELECT days.*
             FROM days
             INNER JOIN users ON users.id = days.user_id
             WHERE users.name = $1
             ORDER BY days.date DESC, days.id DESC",
            name,
        )
        .fetch_all(pool)
        .await?;

        Ok(days)
    }

    pub async fn find_one_by_user(
        value: &FindDayByUserSchema,
        pool: &PgPool,
    ) -> Result<Self, ServerError> {
        let FindDayByUserSchema { user_name, date } = value;

        let day = sqlx::query_as!(
            Self,
            "SELECT days.*
             FROM days
             INNER JOIN users ON users.id = days.user_id
             WHERE users.name = $1 AND days.date = $2",
            user_name,
            date,
        )
        .fetch_one(pool)
        .await?;

        Ok(day)
    }

    pub async fn create(value: &CreateDaySchema, pool: &PgPool) -> Result<Self, ServerError> {
        let CreateDaySchema {
            date,
            user_name,
            weight_kg,
            target_calories,
            notes,
        } = value;

        let day = sqlx::query_as!(
            Self,
            "INSERT INTO days (user_id, date, weight_kg, target_calories, notes)
             VALUES ((SELECT id FROM users WHERE name = $1), $2, $3, $4, $5)
             RETURNING *",
            user_name,
            date,
            weight_kg.as_ref(),
            target_calories,
            notes.as_deref(),
        )
        .fetch_one(pool)
        .await?;

        Ok(day)
    }

    pub async fn update_weight_kg(
        value: UpdateDayWeightSchema,
        pool: &PgPool,
    ) -> Result<Self, ServerError> {
        let UpdateDayWeightSchema {
            user_name,
            date,
            weight_kg,
        } = value;

        let day = sqlx::query_as!(
            Self,
            "UPDATE days
             SET weight_kg = $1
             FROM users
             WHERE users.id = days.user_id
               AND users.name = $2
               AND days.date = $3
             RETURNING days.*",
            weight_kg.as_ref(),
            user_name,
            date,
        )
        .fetch_one(pool)
        .await?;

        Ok(day)
    }

    pub async fn update_target_calories(
        value: UpdateDayTargetCaloriesSchema,
        pool: &PgPool,
    ) -> Result<Self, ServerError> {
        let UpdateDayTargetCaloriesSchema {
            user_name,
            date,
            target_calories,
        } = value;

        let day = sqlx::query_as!(
            Self,
            "UPDATE days
             SET target_calories = $1
             FROM users
             WHERE users.id = days.user_id
               AND users.name = $2
               AND days.date = $3
             RETURNING days.*",
            target_calories,
            user_name,
            date,
        )
        .fetch_one(pool)
        .await?;

        Ok(day)
    }

    pub async fn update_notes(
        value: UpdateDayNotesSchema,
        pool: &PgPool,
    ) -> Result<Self, ServerError> {
        let UpdateDayNotesSchema {
            user_name,
            date,
            notes,
        } = value;

        let day = sqlx::query_as!(
            Self,
            "UPDATE days
             SET notes = $1
             FROM users
             WHERE users.id = days.user_id
               AND users.name = $2
               AND days.date = $3
             RETURNING days.*",
            notes.as_deref(),
            user_name,
            date,
        )
        .fetch_one(pool)
        .await?;

        Ok(day)
    }
}
