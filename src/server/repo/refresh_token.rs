use chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use sqlx::PgPool;

use crate::server::error::ServerError;

pub struct RefreshToken {
    pub id: i64,
    pub user_id: i32,
    pub token_hash: String,
    pub family_id: Uuid,
    pub expires_at: DateTime<Utc>,
    pub used_at: Option<DateTime<Utc>>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

pub struct CreateRefreshToken {
    pub user_id: i32,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
}

/// The token consumed by a rotation, carrying exactly what's needed to issue
/// its replacement (`user_id`, `family_id`) and to re-authenticate the user
/// (`user_name`).
struct ConsumedRefreshToken {
    user_id: i32,
    user_name: String,
    family_id: Uuid,
}

pub struct RotatedRefreshToken {
    pub user_name: String,
}

impl RefreshToken {
    pub async fn create(value: &CreateRefreshToken, pool: &PgPool) -> Result<Self, ServerError> {
        let CreateRefreshToken {
            user_id,
            token_hash,
            expires_at,
        } = value;

        let refresh_token = sqlx::query_as!(
            Self,
            "INSERT INTO refresh_tokens (user_id, token_hash, expires_at)
             VALUES ($1, $2, $3)
             RETURNING *",
            user_id,
            token_hash,
            expires_at,
        )
        .fetch_one(pool)
        .await?;

        Ok(refresh_token)
    }

    /// Consumes one refresh token and creates its replacement atomically.
    ///
    /// Scoped to the token's own family: other families for the same user
    /// (other devices, other agents) are untouched by this call.
    ///
    /// If the token was already consumed, every token in *this* family is
    /// revoked. This turns reuse into session invalidation instead of letting
    /// a single stolen token fork into two independently valid chains.
    pub async fn rotate(
        current_token_hash: &str,
        next_token_hash: &str,
        next_expires_at: DateTime<Utc>,
        pool: &PgPool,
    ) -> Result<Option<RotatedRefreshToken>, ServerError> {
        let mut transaction = pool.begin().await?;

        let consumed = sqlx::query_as!(
            ConsumedRefreshToken,
            r#"UPDATE refresh_tokens AS refresh_token
               SET used_at = now()
               FROM users
               WHERE refresh_token.token_hash = $1
                 AND refresh_token.user_id = users.id
                 AND refresh_token.used_at IS NULL
                 AND refresh_token.revoked_at IS NULL
                 AND refresh_token.expires_at > now()
               RETURNING refresh_token.user_id,
                         users.name AS user_name,
                         refresh_token.family_id"#,
            current_token_hash,
        )
        .fetch_optional(&mut *transaction)
        .await?;

        let Some(ConsumedRefreshToken {
            user_id,
            user_name,
            family_id,
        }) = consumed
        else {
            // A known token that can no longer be consumed indicates reuse.
            // Keep the response indistinguishable from an unknown token.
            sqlx::query!(
                "UPDATE refresh_tokens
                 SET revoked_at = COALESCE(revoked_at, now())
                 WHERE family_id = (
                     SELECT family_id
                     FROM refresh_tokens
                     WHERE token_hash = $1
                 )",
                current_token_hash,
            )
            .execute(&mut *transaction)
            .await?;

            transaction.commit().await?;
            return Ok(None);
        };

        sqlx::query!(
            "INSERT INTO refresh_tokens (user_id, token_hash, family_id, expires_at)
             VALUES ($1, $2, $3, $4)",
            user_id,
            next_token_hash,
            family_id,
            next_expires_at,
        )
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(Some(RotatedRefreshToken { user_name }))
    }

    pub async fn delete_family_by_token_hash(
        token_hash: &str,
        pool: &PgPool,
    ) -> Result<(), ServerError> {
        sqlx::query!(
            "DELETE FROM refresh_tokens
             WHERE family_id = (
                 SELECT family_id
                 FROM refresh_tokens
                 WHERE token_hash = $1
             )",
            token_hash,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn delete_expired(pool: &PgPool) -> Result<u64, ServerError> {
        let result = sqlx::query!("DELETE FROM refresh_tokens WHERE expires_at <= now()")
            .execute(pool)
            .await?;

        Ok(result.rows_affected())
    }
}
