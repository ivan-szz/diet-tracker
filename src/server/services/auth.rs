use argon2::password_hash::Error;
use crate::schema::user::UserSchema;
use crate::schema::user::{LoginUserSchema, RegisterUserSchema};
use crate::server::error::ServerError;
use crate::server::repo::user::{RegisterUser, User};
use crate::utils::argon::{hash, verify};
use crate::utils::jwt::{generate_jwt, Claims};
use sqlx::PgPool;

pub async fn get_authenticated_user(
    claims: &Claims,
    pool: &PgPool,
) -> Result<UserSchema, ServerError> {
    let Some(user) = User::find_by_name(&claims.sub, pool).await? else {
        return Err(ServerError::Unauthorized);
    };

    Ok(user.into())
}

pub async fn login(payload: &LoginUserSchema, pool: &PgPool) -> Result<String, ServerError> {
    let Some(user) = User::find_by_name(&payload.name, pool).await? else {
        return Err(ServerError::Unauthorized);
    };

    compare_passwords(&user.password_hash, &payload.password)?;

    let claims: Claims = (&user).into();
    let token = generate_jwt(&claims)?;
    Ok(token)
}

pub async fn register(payload: &RegisterUserSchema, pool: &PgPool) -> Result<String, ServerError> {
    let existing_user = User::find_by_name(&payload.name, pool).await?;

    if existing_user.is_some() {
        return Err(ServerError::UserAlreadyExists);
    }

    let user_record = RegisterUser {
        name: payload.name.clone(),
        password_hash: hash(&payload.password)?,
    };

    let user = User::create(&user_record, pool).await?;
    let claims: Claims = (&user).into();
    let jwt = generate_jwt(&claims)?;
    Ok(jwt)
}

pub fn compare_passwords(
    password_hash: &str,
    password: &str,
) -> Result<(), ServerError> {
    match verify(password, password_hash) {
        Ok(()) => Ok(()),
        Err(Error::Password) => Err(ServerError::InvalidCredentials),
        Err(error) => Err(ServerError::Hashing(error)),
    }
}
