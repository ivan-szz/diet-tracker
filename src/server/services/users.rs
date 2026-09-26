use crate::schema::user::{UpdateUserStreakSchema, UpdateUserTargetWeightSchema, UserSchema};
use crate::server::error::ServerError;
use crate::server::repo::user::{UpdateStreak, UpdateTargetWeight, User};
use sqlx::PgPool;

pub async fn list(pool: &PgPool) -> Result<Vec<UserSchema>, ServerError> {
    let users = User::find_all(pool).await?;
    Ok(users.into_iter().map(UserSchema::from).collect())
}

/// Fails with [`ServerError::UserNotFound`], so reads for a mistyped name
/// aren't mistaken for a user with no data.
pub async fn find(name: &str, pool: &PgPool) -> Result<UserSchema, ServerError> {
    User::find_by_name(name, pool)
        .await?
        .map(UserSchema::from)
        .ok_or(ServerError::UserNotFound)
}

pub async fn update_target_weight(
    payload: &UpdateUserTargetWeightSchema,
    pool: &PgPool,
) -> Result<UserSchema, ServerError> {
    let user = User::update_target_weight(
        &UpdateTargetWeight {
            name: payload.name.clone(),
            target_weight_kg: payload.target_weight_kg,
        },
        pool,
    )
    .await?;

    Ok(user.into())
}

pub async fn update_streak(
    payload: &UpdateUserStreakSchema,
    pool: &PgPool,
) -> Result<UserSchema, ServerError> {
    let user = User::update_streak(
        &UpdateStreak {
            name: payload.name.clone(),
            streak: payload.streak,
        },
        pool,
    )
    .await?;

    Ok(user.into())
}
