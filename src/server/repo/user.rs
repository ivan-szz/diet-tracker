use chrono::{DateTime, Utc};
use sqlx::PgPool;

use crate::{
    schema::user::{RegisterUserSchema, UpdateUserStreakSchema, UpdateUserTargetWeightSchema},
    server::error::ServerError,
    utils::argon::{hash, verify},
};

pub struct User {
    pub id: i32,
    pub name: String,
    pub password_hash: Option<String>,
    pub streak: i32,
    pub target_weight_kg: f32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<Self>, ServerError> {
        let users = sqlx::query_as!(Self, "SELECT * FROM users")
            .fetch_all(pool)
            .await?;

        Ok(users)
    }

    pub async fn create(value: &RegisterUserSchema, pool: &PgPool) -> Result<Self, ServerError> {
        let RegisterUserSchema { name, password } = value;

        let hashed_password = hash(password)?;

        let user = sqlx::query_as!(
            Self,
            "INSERT INTO users (name, password_hash) VALUES ($1, $2) RETURNING *",
            name,
            hashed_password
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn update_target_weight(
        value: &UpdateUserTargetWeightSchema,
        pool: &PgPool,
    ) -> Result<Self, ServerError> {
        let UpdateUserTargetWeightSchema {
            name,
            target_weight_kg,
        } = value;

        let user = sqlx::query_as!(
            Self,
            "UPDATE users SET target_weight_kg = $1 WHERE name = $2 RETURNING *",
            target_weight_kg,
            name
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn update_streak(
        value: &UpdateUserStreakSchema,
        pool: &PgPool,
    ) -> Result<Self, ServerError> {
        let UpdateUserStreakSchema { name, streak } = value;

        let user = sqlx::query_as!(
            Self,
            "UPDATE users SET streak = $1 WHERE name = $2 RETURNING *",
            streak,
            name
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub fn verify_password(&self, password: &str) -> Result<(), ServerError> {
        let hash = self.password_hash.as_deref().unwrap_or_default();
        verify(password, hash)?;
        Ok(())
    }
}
