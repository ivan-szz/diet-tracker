use crate::schema::user::UserSchema;
use crate::server::error::ServerError;
use crate::server::services::auth;
use crate::utils::jwt::verify_jwt;
use dioxus::fullstack::{Cookie, TypedHeader};
use sqlx::PgPool;

pub async fn current_user_from_cookie(
    cookie: &TypedHeader<Cookie>,
    pool: &PgPool,
) -> dioxus::Result<UserSchema, ServerError> {
    let token = cookie.get("session").ok_or(ServerError::Unauthorized)?;
    let claims = verify_jwt(token)?;
    let user = auth::get_authenticated_user(&claims, &pool).await?;
    Ok(user)
}

pub fn session_cookie_attrs() -> &'static str {
    if cfg!(debug_assertions) {
        "SameSite=Lax; Path=/; HttpOnly"
    } else {
        "SameSite=Lax; Path=/; HttpOnly; Secure"
    }
}

pub fn refresh_cookie_attrs() -> &'static str {
    if cfg!(debug_assertions) {
        "SameSite=Lax; Path=/api/auth; HttpOnly"
    } else {
        "SameSite=Lax; Path=/api/auth; HttpOnly; Secure"
    }
}

pub fn ensure_owner(name: &str, current_user: &UserSchema) -> Result<(), ServerError> {
    if name == current_user.name {
        Ok(())
    } else {
        Err(ServerError::Unauthorized)
    }
}
