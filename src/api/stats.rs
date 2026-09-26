use crate::schema::stats::{
    CommunityQuerySchema, SummaryQuerySchema, TrendPointSchema, TrendQuerySchema, UserSummarySchema,
};
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::server::services::stats;
#[cfg(feature = "server")]
use crate::server::session::current_user_from_cookie;
#[cfg(feature = "server")]
use chrono::{Days, Local};
#[cfg(feature = "server")]
use dioxus::fullstack::{headers::Cookie, TypedHeader};
#[cfg(feature = "server")]
use dioxus::server::axum::Extension;
#[cfg(feature = "server")]
use sqlx::PgPool;

/// A user's calorie progress for a date plus their weight progress. Progress
/// is public read data for every authenticated user, not just its owner.
#[get("/api/stats/summary?:query", cookie: TypedHeader<Cookie>, pool: Extension<PgPool>)]
pub async fn summary(query: SummaryQuerySchema) -> ServerFnResult<UserSummarySchema> {
    let current_user = current_user_from_cookie(&cookie, &pool).await?;
    let user_name = query.user_name.unwrap_or(current_user.name);
    let date = query.date.unwrap_or_else(|| Local::now().date_naive());

    Ok(stats::summary(&user_name, date, &pool).await?)
}

/// Every user's summary, as in [`summary`].
#[get("/api/stats/community?:query", cookie: TypedHeader<Cookie>, pool: Extension<PgPool>)]
pub async fn community(query: CommunityQuerySchema) -> ServerFnResult<Vec<UserSummarySchema>> {
    current_user_from_cookie(&cookie, &pool).await?;
    let date = query.date.unwrap_or_else(|| Local::now().date_naive());

    Ok(stats::community(date, &pool).await?)
}

/// Daily calories, calorie target and weight for a user over a date range.
#[get("/api/stats/trend?:query", cookie: TypedHeader<Cookie>, pool: Extension<PgPool>)]
pub async fn trend(query: TrendQuerySchema) -> ServerFnResult<Vec<TrendPointSchema>> {
    let current_user = current_user_from_cookie(&cookie, &pool).await?;
    let user_name = query.user_name.unwrap_or(current_user.name);
    let to = query.to.unwrap_or_else(|| Local::now().date_naive());
    let from = query.from.unwrap_or(to - Days::new(29));

    Ok(stats::trend(&user_name, from, to, &pool).await?)
}
