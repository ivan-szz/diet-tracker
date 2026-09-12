use crate::schema::user::{UpdateUserStreakSchema, UpdateUserTargetWeightSchema, UserSchema};
use dioxus::prelude::*;
#[cfg(feature = "server")]
use validator::Validate;

#[cfg(feature = "server")]
use crate::server::error::ServerError;
#[cfg(feature = "server")]
use crate::server::services::users;
#[cfg(feature = "server")]
use crate::server::session::{current_user_from_cookie, ensure_owner};
#[cfg(feature = "server")]
use dioxus::fullstack::{headers::Cookie, TypedHeader};
#[cfg(feature = "server")]
use dioxus::server::axum::Extension;
#[cfg(feature = "server")]
use sqlx::PgPool;

/// Returns the community progress visible to every authenticated user.
#[get("/api/users", cookie: TypedHeader<Cookie>, pool: Extension<PgPool>)]
pub async fn list() -> ServerFnResult<Vec<UserSchema>> {
    current_user_from_cookie(&cookie, &pool).await?;
    Ok(users::list(&pool).await?)
}

#[post("/api/users/target-weight", cookie: TypedHeader<Cookie>, pool: Extension<PgPool>)]
pub async fn update_target_weight(
    payload: UpdateUserTargetWeightSchema,
) -> ServerFnResult<UserSchema> {
    payload.validate().map_err(ServerError::from)?;
    let current_user = current_user_from_cookie(&cookie, &pool).await?;
    ensure_owner(&payload.name, &current_user)?;

    Ok(users::update_target_weight(&payload, &pool).await?)
}

#[post("/api/users/streak", cookie: TypedHeader<Cookie>, pool: Extension<PgPool>)]
pub async fn update_streak(payload: UpdateUserStreakSchema) -> ServerFnResult<UserSchema> {
    payload.validate().map_err(ServerError::from)?;
    let current_user = current_user_from_cookie(&cookie, &pool).await?;
    ensure_owner(&payload.name, &current_user)?;

    Ok(users::update_streak(&payload, &pool).await?)
}
