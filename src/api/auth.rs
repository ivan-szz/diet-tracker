use crate::schema::auth::AuthTokensSchema;
use crate::schema::user::{LoginUserSchema, RegisterUserSchema, UserSchema};
#[cfg(feature = "server")]
use crate::server::error::ServerError;
#[cfg(feature = "server")]
use crate::server::services::auth;
#[cfg(feature = "server")]
use crate::server::session::{
    current_user_from_cookie, refresh_cookie_attrs, session_cookie_attrs,
};
#[cfg(feature = "server")]
use dioxus::fullstack::headers::Cookie;
#[cfg(feature = "server")]
use dioxus::fullstack::TypedHeader;
use dioxus::fullstack::{SetCookie, SetHeader};
use dioxus::prelude::*;
#[cfg(feature = "server")]
use dioxus::server::axum::Extension;
#[cfg(feature = "server")]
use sqlx::PgPool;
use validator::Validate;

pub type AuthCookieHeaders = (SetHeader<SetCookie>, SetHeader<SetCookie>);

#[post("/api/auth/login", pool: Extension<PgPool>)]
pub async fn login(payload: LoginUserSchema) -> ServerFnResult<AuthCookieHeaders> {
    payload.validate().map_err(ServerError::from)?;
    let tokens = auth::login(&payload, &pool).await?;
    auth_cookie_headers(tokens)
}

#[post("/api/auth/register", pool: Extension<PgPool>)]
pub async fn register(payload: RegisterUserSchema) -> ServerFnResult<AuthCookieHeaders> {
    payload.validate().map_err(ServerError::from)?;
    let tokens = auth::register(&payload, &pool).await?;

    auth_cookie_headers(tokens)
}

#[post("/api/auth/logout", cookie: TypedHeader<Cookie>, pool: Extension<PgPool>)]
pub async fn logout() -> ServerFnResult<AuthCookieHeaders> {
    auth::logout(cookie.get("refresh_token"), &pool).await?;

    clear_auth_cookie_headers()
}

#[get("/api/auth/me", cookie: TypedHeader<Cookie>, pool: Extension<PgPool>)]
pub async fn me() -> ServerFnResult<UserSchema> {
    let user = current_user_from_cookie(&cookie, &pool).await?;

    Ok(user)
}

#[post("/api/auth/refresh", cookie: TypedHeader<Cookie>, pool: Extension<PgPool>)]
pub async fn refresh() -> ServerFnResult<AuthCookieHeaders> {
    let refresh_token = cookie
        .get("refresh_token")
        .ok_or(ServerError::Unauthorized)?;
    let tokens = auth::refresh(refresh_token, &pool).await?;

    auth_cookie_headers(tokens)
}

#[cfg(feature = "server")]
fn auth_cookie_headers(tokens: AuthTokensSchema) -> ServerFnResult<AuthCookieHeaders> {
    Ok((
        SetHeader::<SetCookie>::new(format!(
            "session={}; {};",
            tokens.access_token,
            session_cookie_attrs(),
        ))
        .map_err(ServerFnError::new)?,
        SetHeader::<SetCookie>::new(format!(
            "refresh_token={}; {};",
            tokens.refresh_token,
            refresh_cookie_attrs()
        ))
        .map_err(ServerFnError::new)?,
    ))
}

#[cfg(feature = "server")]
fn clear_auth_cookie_headers() -> ServerFnResult<AuthCookieHeaders> {
    let expired = "Max-Age=0; Expires=Thu, 01 Jan 1970 00:00:00 GMT";

    Ok((
        SetHeader::<SetCookie>::new(format!("session=; {}; {expired}", session_cookie_attrs()))
            .map_err(ServerFnError::new)?,
        SetHeader::<SetCookie>::new(format!(
            "refresh_token=; {}; {expired}",
            refresh_cookie_attrs()
        ))
        .map_err(ServerFnError::new)?,
    ))
}
