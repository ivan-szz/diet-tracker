use crate::schema::user::{UpdateUserTargetWeightSchema, UserSchema};
use crate::server::error::ServerError;
use crate::server::repo::day::{Day, TrackedDay};
use crate::server::repo::user::{UpdateTargetWeight, User};
use crate::server::services::streak::{self, Streak};
use chrono::Local;
use sqlx::PgPool;

pub async fn list(pool: &PgPool) -> Result<Vec<UserSchema>, ServerError> {
    let users = User::find_all(pool).await?;
    let tracked = Day::find_tracked(None, pool).await?;

    Ok(users
        .into_iter()
        .map(|user| {
            let streak = streak_of(user.id, &tracked);
            user.into_schema(streak)
        })
        .collect())
}

/// Fails with [`ServerError::UserNotFound`], so reads for a mistyped name
/// aren't mistaken for a user with no data.
pub async fn find(name: &str, pool: &PgPool) -> Result<UserSchema, ServerError> {
    let user = User::find_by_name(name, pool)
        .await?
        .ok_or(ServerError::UserNotFound)?;

    with_streak(user, pool).await
}

pub async fn with_streak(user: User, pool: &PgPool) -> Result<UserSchema, ServerError> {
    let tracked = Day::find_tracked(Some(user.id), pool).await?;
    let streak = streak_of(user.id, &tracked);

    Ok(user.into_schema(streak))
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

    with_streak(user, pool).await
}

fn streak_of(user_id: i32, tracked: &[TrackedDay]) -> Streak {
    streak::compute(
        Local::now().date_naive(),
        tracked
            .iter()
            .filter(|day| day.user_id == user_id)
            .map(|day| (day.date, day.tracked_at.with_timezone(&Local).date_naive())),
    )
}
