use chrono::{DateTime, Utc};
use sqlx::PgPool;

use crate::schema::user::UserSchema;
use crate::server::error::ServerError;

pub struct User {
    pub id: i32,
    pub name: String,
    pub password_hash: String,
    pub streak: i32,
    pub target_weight_kg: Option<f32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct RegisterUser {
    pub name: String,
    pub password_hash: String,
}

pub struct UpdateTargetWeight {
    pub name: String,
    pub target_weight_kg: f32,
}

pub struct UpdateStreak {
    pub name: String,
    pub streak: i32,
}

impl From<User> for UserSchema {
    fn from(value: User) -> Self {
        UserSchema {
            id: value.id,
            name: value.name,
            streak: value.streak,
            target_weight_kg: value.target_weight_kg,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl User {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<Self>, ServerError> {
        let users = sqlx::query_as!(Self, "SELECT * FROM users")
            .fetch_all(pool)
            .await?;

        Ok(users)
    }

    pub async fn find_by_name(name: &str, pool: &PgPool) -> Result<Option<Self>, ServerError> {
        let user = sqlx::query_as!(Self, "SELECT * FROM users WHERE name = $1", name)
            .fetch_optional(pool)
            .await?;

        Ok(user)
    }

    pub async fn create(value: &RegisterUser, pool: &PgPool) -> Result<Self, ServerError> {
        let RegisterUser {
            name,
            password_hash,
        } = value;

        let user = sqlx::query_as!(
            Self,
            "INSERT INTO users (name, password_hash) VALUES ($1, $2) RETURNING *",
            name,
            password_hash
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn update_target_weight(
        value: &UpdateTargetWeight,
        pool: &PgPool,
    ) -> Result<Self, ServerError> {
        let UpdateTargetWeight {
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

    pub async fn update_streak(value: &UpdateStreak, pool: &PgPool) -> Result<Self, ServerError> {
        let UpdateStreak { name, streak } = value;

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
}
