use chrono::{DateTime, Utc};
use sqlx::PgPool;

use crate::schema::user::UserSchema;
use crate::server::error::ServerError;
use crate::server::services::streak::Streak;

pub struct User {
    pub id: i32,
    pub name: String,
    pub password_hash: String,
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

impl User {
    pub fn into_schema(self, streak: Streak) -> UserSchema {
        UserSchema {
            id: self.id,
            name: self.name,
            streak: streak.days,
            streak_at_risk: streak.at_risk,
            target_weight_kg: self.target_weight_kg,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }

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
}
