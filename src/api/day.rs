use crate::schema::day::{
    CreateDaySchema, DaySchema, DeleteDaySchema, UpdateDayNotesSchema,
    UpdateDayTargetCaloriesSchema, UpdateDayWeightSchema,
};
use dioxus::prelude::*;
#[cfg(feature = "server")]
use validator::Validate;

#[cfg(feature = "server")]
use crate::server::error::ServerError;
#[cfg(feature = "server")]
use crate::server::services::day;
#[cfg(feature = "server")]
use crate::server::session::{current_user_from_cookie, ensure_owner};
#[cfg(feature = "server")]
use dioxus::fullstack::{headers::Cookie, TypedHeader};
#[cfg(feature = "server")]
use dioxus::server::axum::Extension;
#[cfg(feature = "server")]
use sqlx::PgPool;

#[get("/api/days", cookie: TypedHeader<Cookie>, pool: Extension<PgPool>)]
pub async fn list() -> ServerFnResult<Vec<DaySchema>> {
    let current_user = current_user_from_cookie(&cookie, &pool).await?;

    Ok(day::list(&current_user.name, &pool).await?)
}

/// Returns the given user's days; day history is public read data for every
/// authenticated user, not just its owner.
#[get("/api/days/community?user_name", cookie: TypedHeader<Cookie>, pool: Extension<PgPool>)]
pub async fn list_for_user(user_name: String) -> ServerFnResult<Vec<DaySchema>> {
    current_user_from_cookie(&cookie, &pool).await?;

    Ok(day::list(&user_name, &pool).await?)
}

#[post("/api/days", cookie: TypedHeader<Cookie>, pool: Extension<PgPool>)]
pub async fn create(payload: CreateDaySchema) -> ServerFnResult<DaySchema> {
    payload.validate().map_err(ServerError::from)?;
    let current_user = current_user_from_cookie(&cookie, &pool).await?;
    ensure_owner(&payload.user_name, &current_user)?;

    Ok(day::create(&payload, &pool).await?)
}

#[post("/api/days/weight", cookie: TypedHeader<Cookie>, pool: Extension<PgPool>)]
pub async fn update_weight(payload: UpdateDayWeightSchema) -> ServerFnResult<DaySchema> {
    payload.validate().map_err(ServerError::from)?;
    let current_user = current_user_from_cookie(&cookie, &pool).await?;
    ensure_owner(&payload.user_name, &current_user)?;

    Ok(day::update_weight(payload, &pool).await?)
}

#[post("/api/days/target-calories", cookie: TypedHeader<Cookie>, pool: Extension<PgPool>)]
pub async fn update_target_calories(
    payload: UpdateDayTargetCaloriesSchema,
) -> ServerFnResult<DaySchema> {
    payload.validate().map_err(ServerError::from)?;
    let current_user = current_user_from_cookie(&cookie, &pool).await?;
    ensure_owner(&payload.user_name, &current_user)?;

    Ok(day::update_target_calories(payload, &pool).await?)
}

#[post("/api/days/notes", cookie: TypedHeader<Cookie>, pool: Extension<PgPool>)]
pub async fn update_notes(payload: UpdateDayNotesSchema) -> ServerFnResult<DaySchema> {
    let current_user = current_user_from_cookie(&cookie, &pool).await?;
    ensure_owner(&payload.user_name, &current_user)?;

    Ok(day::update_notes(payload, &pool).await?)
}

#[post("/api/days/delete", cookie: TypedHeader<Cookie>, pool: Extension<PgPool>)]
pub async fn delete(payload: DeleteDaySchema) -> ServerFnResult<()> {
    let current_user = current_user_from_cookie(&cookie, &pool).await?;
    ensure_owner(&payload.user_name, &current_user)?;

    Ok(day::delete(&payload, &pool).await?)
}
