use crate::schema::user::{LoginUserSchema, RegisterUserSchema, UserSchema};
use dioxus::fullstack::headers::Cookie;
use dioxus::fullstack::{SetCookie, SetHeader};
use dioxus::prelude::*;
use validator::Validate;

#[cfg(feature = "server")]
use crate::server::error::ServerError;
#[cfg(feature = "server")]
use crate::server::services::auth;
#[cfg(feature = "server")]
use crate::server::session::{current_user_from_cookie, session_cookie_attrs};
#[cfg(feature = "server")]
use dioxus::fullstack::TypedHeader;
#[cfg(feature = "server")]
use dioxus::server::axum::Extension;
#[cfg(feature = "server")]
use sqlx::PgPool;

#[post("/api/auth/login", pool: Extension<PgPool>)]
pub async fn login(payload: LoginUserSchema) -> ServerFnResult<SetHeader<SetCookie>> {
    payload.validate().map_err(ServerError::from)?;
    let token = auth::login(&payload, &pool).await?;
    SetHeader::<SetCookie>::new(format!("session={token}; {}", session_cookie_attrs()))
        .map_err(ServerFnError::new)
}

#[post("/api/auth/register", pool: Extension<PgPool>)]
pub async fn register(payload: RegisterUserSchema) -> ServerFnResult<SetHeader<SetCookie>> {
    payload.validate().map_err(ServerError::from)?;
    let token = auth::register(&payload, &pool).await?;

    SetHeader::<SetCookie>::new(format!("session={token}; {}", session_cookie_attrs()))
        .map_err(ServerFnError::new)
}

#[post("/api/auth/logout", cookie: TypedHeader<Cookie>, pool: Extension<PgPool>)]
pub async fn logout() -> ServerFnResult<SetHeader<SetCookie>> {
    current_user_from_cookie(&cookie, &pool).await?;

    SetHeader::<SetCookie>::new(format!(
        "session=; {}; Max-Age=0; Expires=Thu, 01 Jan 1970 00:00:00 GMT",
        session_cookie_attrs()
    ))
    .map_err(ServerFnError::new)
}

#[get("/api/auth/me", cookie: TypedHeader<Cookie>, pool: Extension<PgPool>)]
pub async fn me() -> ServerFnResult<UserSchema> {
    let user = current_user_from_cookie(&cookie, &pool).await?;

    Ok(user)
}
