use crate::schema::entry::{
    CreateEntrySchema, DeleteEntrySchema, EntrySchema, UpdateEntryNotesSchema,
};
use dioxus::prelude::*;
#[cfg(feature = "server")]
use validator::Validate;

#[cfg(feature = "server")]
use crate::server::error::ServerError;
#[cfg(feature = "server")]
use crate::server::services::entry;
#[cfg(feature = "server")]
use crate::server::session::{current_user_from_cookie, ensure_owner};
#[cfg(feature = "server")]
use dioxus::fullstack::{headers::Cookie, TypedHeader};
#[cfg(feature = "server")]
use dioxus::server::axum::Extension;
#[cfg(feature = "server")]
use sqlx::PgPool;

#[get("/api/entries", cookie: TypedHeader<Cookie>, pool: Extension<PgPool>)]
pub async fn list() -> ServerFnResult<Vec<EntrySchema>> {
    let current_user = current_user_from_cookie(&cookie, &pool).await?;

    Ok(entry::list(&current_user.name, &pool).await?)
}

/// Returns the given user's entries; entry history is public read data for
/// every authenticated user, not just its owner.
#[get("/api/entries/community?user_name", cookie: TypedHeader<Cookie>, pool: Extension<PgPool>)]
pub async fn list_for_user(user_name: String) -> ServerFnResult<Vec<EntrySchema>> {
    current_user_from_cookie(&cookie, &pool).await?;

    Ok(entry::list(&user_name, &pool).await?)
}

#[post("/api/entries", cookie: TypedHeader<Cookie>, pool: Extension<PgPool>)]
pub async fn create(payload: CreateEntrySchema) -> ServerFnResult<EntrySchema> {
    payload.validate().map_err(ServerError::from)?;
    let current_user = current_user_from_cookie(&cookie, &pool).await?;
    ensure_owner(&payload.user_name, &current_user)?;

    Ok(entry::create(&payload, &pool).await?)
}

#[post("/api/entries/notes", cookie: TypedHeader<Cookie>, pool: Extension<PgPool>)]
pub async fn update_notes(payload: UpdateEntryNotesSchema) -> ServerFnResult<EntrySchema> {
    let current_user = current_user_from_cookie(&cookie, &pool).await?;
    ensure_owner(&payload.user_name, &current_user)?;

    Ok(entry::update_notes(&payload, &pool).await?)
}

#[post("/api/entries/delete", cookie: TypedHeader<Cookie>, pool: Extension<PgPool>)]
pub async fn delete(payload: DeleteEntrySchema) -> ServerFnResult<()> {
    let current_user = current_user_from_cookie(&cookie, &pool).await?;
    ensure_owner(&payload.user_name, &current_user)?;

    Ok(entry::delete(&payload, &pool).await?)
}
