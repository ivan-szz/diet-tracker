use chrono::{DateTime, NaiveDate, Utc};
use sqlx::PgPool;

use crate::{
    schema::day::{
        DayFiltersSchema, DayOutcome, DaySchema, DeleteDaySchema, FindDayByUserSchema,
        UpdateDayNotesSchema, UpdateDayTargetCaloriesSchema, UpdateDayWeightSchema,
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
    pub weight_logged_at: Option<DateTime<Utc>>,
}

/// A day with a weight or at least one entry, and when the first of them was
/// logged.
pub struct TrackedDay {
    pub user_id: i32,
    pub date: NaiveDate,
    pub tracked_at: DateTime<Utc>,
}

pub struct CreateDay {
    pub date: NaiveDate,
    pub user_name: String,
    pub weight_kg: Option<f32>,
    pub target_calories: i32,
    pub notes: Option<String>,
}

impl From<Day> for DaySchema {
    fn from(value: Day) -> Self {
        DaySchema {
            id: value.id,
            date: value.date,
            user_id: value.user_id,
            weight_kg: value.weight_kg,
            target_calories: value.target_calories,
            notes: value.notes,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl Day {
    pub async fn find_by_user(
        name: &str,
        filters: &DayFiltersSchema,
        pool: &PgPool,
    ) -> Result<Vec<Self>, ServerError> {
        let DayFiltersSchema {
            from,
            to,
            outcome,
            q,
        } = filters;

        let days = sqlx::query_as!(
            Self,
            "SELECT days.*
             FROM days
             INNER JOIN users ON users.id = days.user_id
             WHERE users.name = $1
               AND ($2::date IS NULL OR days.date >= $2)
               AND ($3::date IS NULL OR days.date <= $3)
               AND ($4::text IS NULL OR ($4 = 'over') = (
                   COALESCE((
                       SELECT SUM(entries.calories)
                       FROM entries
                       WHERE entries.user_id = days.user_id AND entries.date = days.date
                   ), 0) > days.target_calories
               ))
               AND ($5::text IS NULL OR EXISTS (
                   SELECT 1
                   FROM entries
                   WHERE entries.user_id = days.user_id
                     AND entries.date = days.date
                     AND strpos(
                         lower(entries.name || ' ' || COALESCE(entries.notes, '')),
                         lower($5)
                     ) > 0
               ))
             ORDER BY days.date DESC, days.id DESC",
            name,
            *from,
            *to,
            outcome.map(DayOutcome::as_str),
            q.as_deref(),
        )
        .fetch_all(pool)
        .await?;

        Ok(days)
    }

    pub async fn find_one_by_user(
        value: &FindDayByUserSchema,
        pool: &PgPool,
    ) -> Result<Option<Self>, ServerError> {
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
        .fetch_optional(pool)
        .await?;

        Ok(day)
    }

    pub async fn find_previous(
        user_name: &str,
        date: NaiveDate,
        pool: &PgPool,
    ) -> Result<Option<Self>, ServerError> {
        let day = sqlx::query_as!(
            Self,
            "SELECT days.*
             FROM days
             INNER JOIN users ON users.id = days.user_id
             WHERE users.name = $1 AND days.date < $2
             ORDER BY days.date DESC
             LIMIT 1",
            user_name,
            date,
        )
        .fetch_optional(pool)
        .await?;

        Ok(day)
    }

    /// `user_id` narrows the result to one user; `None` returns everyone's.
    pub async fn find_tracked(
        user_id: Option<i32>,
        pool: &PgPool,
    ) -> Result<Vec<TrackedDay>, ServerError> {
        let days = sqlx::query_as!(
            TrackedDay,
            r#"SELECT days.user_id,
                      days.date,
                      LEAST(days.weight_logged_at, MIN(entries.created_at)) AS "tracked_at!"
               FROM days
               LEFT JOIN entries
                   ON entries.user_id = days.user_id AND entries.date = days.date
               WHERE $1::int IS NULL OR days.user_id = $1
               GROUP BY days.id
               HAVING LEAST(days.weight_logged_at, MIN(entries.created_at)) IS NOT NULL"#,
            user_id,
        )
        .fetch_all(pool)
        .await?;

        Ok(days)
    }

    pub async fn create(value: &CreateDay, pool: &PgPool) -> Result<Self, ServerError> {
        let CreateDay {
            date,
            user_name,
            weight_kg,
            target_calories,
            notes,
        } = value;

        let day = sqlx::query_as!(
            Self,
            "INSERT INTO days (user_id, date, weight_kg, target_calories, notes, weight_logged_at)
             VALUES (
                 (SELECT id FROM users WHERE name = $1), $2, $3, $4, $5,
                 CASE WHEN $3::real IS NULL THEN NULL ELSE now() END
             )
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
             SET weight_kg = $1,
                 weight_logged_at = CASE
                     WHEN $1::real IS NULL THEN NULL
                     ELSE COALESCE(days.weight_logged_at, now())
                 END
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

    pub async fn delete(value: &DeleteDaySchema, pool: &PgPool) -> Result<(), ServerError> {
        let DeleteDaySchema { user_name, date } = value;

        sqlx::query!(
            "DELETE FROM days
             USING users
             WHERE users.id = days.user_id
               AND users.name = $1
               AND days.date = $2",
            user_name,
            date,
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
