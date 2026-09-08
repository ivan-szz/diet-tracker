use crate::schema::user::{UpdateUserStreakSchema, UpdateUserTargetWeightSchema, UserSchema};
use crate::server::error::ServerError;
use crate::server::repo::user::{UpdateStreak, UpdateTargetWeight, User};
use sqlx::PgPool;

pub async fn list(pool: &PgPool) -> Result<Vec<UserSchema>, ServerError> {
    let users = User::find_all(pool).await?;
    Ok(users.into_iter().map(UserSchema::from).collect())
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
