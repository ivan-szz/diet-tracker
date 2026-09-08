use crate::server::error::ServerError;
use crate::server::repo::user::User;
use chrono::{Duration, Utc};
use dioxus::logger::tracing;
use jsonwebtoken::Algorithm;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::OnceLock;

static JWT_SECRET: OnceLock<String> = OnceLock::new();

fn get_jwt_secret() -> &'static str {
    JWT_SECRET.get_or_init(|| env::var("JWT_SECRET").expect("Missing JWT_SECRET"))
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    /// The user's nme
    pub sub: String,
}

impl From<&User> for Claims {
    fn from(value: &User) -> Self {
        let expiration = Utc::now() + Duration::minutes(10);
        Self {
            exp: expiration.timestamp() as usize,
            sub: value.name.to_string(),
        }
    }
}

pub fn generate_jwt(claims: &Claims) -> Result<String, ServerError> {
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        claims,
        &jsonwebtoken::EncodingKey::from_secret(get_jwt_secret().as_ref()),
    )?;
    Ok(token)
}

pub fn verify_jwt(token: &str) -> Result<Claims, ServerError> {
    let claims = jsonwebtoken::decode::<Claims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(get_jwt_secret().as_ref()),
        &jsonwebtoken::Validation::new(Algorithm::HS256),
    )
    .map_err(|e| {
        tracing::warn!("Failed to verify JWT: {}", e);
        ServerError::Unauthorized
    })?;
    Ok(claims.claims)
}
