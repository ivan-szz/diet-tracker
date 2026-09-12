use crate::schema::auth::AuthTokensSchema;
use crate::schema::user::UserSchema;
use crate::schema::user::{LoginUserSchema, RegisterUserSchema};
use crate::server::error::ServerError;
use crate::server::repo::refresh_token::{CreateRefreshToken, RefreshToken};
use crate::server::repo::user::{RegisterUser, User};
use crate::utils::argon::{hash, verify};
use crate::utils::jwt::{generate_jwt, Claims};
use argon2::password_hash::Error;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use chrono::{Duration, Utc};
use dioxus::logger::tracing;
use sha2::{Digest, Sha256};
use sqlx::PgPool;

const REFRESH_TOKEN_TTL: Duration = Duration::days(7);

pub async fn get_authenticated_user(
    claims: &Claims,
    pool: &PgPool,
) -> Result<UserSchema, ServerError> {
    let Some(user) = User::find_by_name(&claims.sub, pool).await? else {
        return Err(ServerError::Unauthorized);
    };

    Ok(user.into())
}

pub async fn login(
    payload: &LoginUserSchema,
    pool: &PgPool,
) -> Result<AuthTokensSchema, ServerError> {
    let Some(user) = User::find_by_name(&payload.name, pool).await? else {
        return Err(ServerError::Unauthorized);
    };

    compare_passwords(&user.password_hash, &payload.password)?;

    issue_tokens(&user, pool).await
}

pub async fn register(
    payload: &RegisterUserSchema,
    pool: &PgPool,
) -> Result<AuthTokensSchema, ServerError> {
    let existing_user = User::find_by_name(&payload.name, pool).await?;

    if existing_user.is_some() {
        return Err(ServerError::UserAlreadyExists);
    }

    let user_record = RegisterUser {
        name: payload.name.clone(),
        password_hash: hash(&payload.password)?,
    };

    let user = User::create(&user_record, pool).await?;
    issue_tokens(&user, pool).await
}

pub async fn refresh(
    current_refresh_token: &str,
    pool: &PgPool,
) -> Result<AuthTokensSchema, ServerError> {
    if let Err(error) = RefreshToken::delete_expired(pool).await {
        tracing::warn!("Failed to delete expired refresh tokens: {error}");
    }

    let next_refresh_token = generate_refresh_token();
    let current_token_hash = hash_refresh_token(current_refresh_token);
    let next_token_hash = hash_refresh_token(&next_refresh_token);
    let next_expires_at = Utc::now() + REFRESH_TOKEN_TTL;

    let Some(rotated_token) =
        RefreshToken::rotate(&current_token_hash, &next_token_hash, next_expires_at, pool).await?
    else {
        return Err(ServerError::Unauthorized);
    };

    let claims = Claims::for_subject(&rotated_token.user_name);

    Ok(AuthTokensSchema {
        access_token: generate_jwt(&claims)?,
        refresh_token: next_refresh_token,
    })
}

pub async fn logout(refresh_token: Option<&str>, pool: &PgPool) -> Result<(), ServerError> {
    if let Some(refresh_token) = refresh_token {
        RefreshToken::delete_family_by_token_hash(&hash_refresh_token(refresh_token), pool).await?;
    }

    Ok(())
}

pub fn compare_passwords(password_hash: &str, password: &str) -> Result<(), ServerError> {
    match verify(password, password_hash) {
        Ok(()) => Ok(()),
        Err(Error::Password) => Err(ServerError::InvalidCredentials),
        Err(error) => Err(ServerError::Hashing(error)),
    }
}

async fn issue_tokens(user: &User, pool: &PgPool) -> Result<AuthTokensSchema, ServerError> {
    let claims: Claims = user.into();
    let jwt = generate_jwt(&claims)?;
    let refresh_token = generate_refresh_token();
    let refresh_expires_at = Utc::now() + REFRESH_TOKEN_TTL;

    RefreshToken::create(
        &CreateRefreshToken {
            user_id: user.id,
            token_hash: hash_refresh_token(&refresh_token),
            expires_at: refresh_expires_at,
        },
        pool,
    )
    .await?;

    Ok(AuthTokensSchema {
        access_token: jwt,
        refresh_token,
    })
}

fn generate_refresh_token() -> String {
    let random_bytes: [u8; 32] = rand::random();
    URL_SAFE_NO_PAD.encode(random_bytes)
}

fn hash_refresh_token(token: &str) -> String {
    URL_SAFE_NO_PAD.encode(Sha256::digest(token.as_bytes()))
}
