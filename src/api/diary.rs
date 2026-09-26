use crate::schema::day::DayQuerySchema;
use crate::schema::diary::DiaryDaySchema;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::server::services::diary;
#[cfg(feature = "server")]
use crate::server::session::current_user_from_cookie;
#[cfg(feature = "server")]
use dioxus::fullstack::{headers::Cookie, TypedHeader};
#[cfg(feature = "server")]
use dioxus::server::axum::Extension;
#[cfg(feature = "server")]
use sqlx::PgPool;

/// A user's food diary: the days matching the optional filters in
/// [`DayQuerySchema`], newest first, each with its entries and calorie total.
/// Diary history is public read data for every authenticated user, not just
/// its owner.
#[get("/api/diary?:query", cookie: TypedHeader<Cookie>, pool: Extension<PgPool>)]
pub async fn list(query: DayQuerySchema) -> ServerFnResult<Vec<DiaryDaySchema>> {
    let current_user = current_user_from_cookie(&cookie, &pool).await?;
    let user_name = query.user_name.unwrap_or(current_user.name);

    Ok(diary::list(&user_name, query.filters, &pool).await?)
}
