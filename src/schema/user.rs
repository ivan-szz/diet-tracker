use chrono::{DateTime, Utc};
use serde::Deserialize;
use validator::Validate;

pub struct UserSchema {
    pub id: i32,
    pub name: String,
    pub streak: i32,
    pub target_weight_kg: f32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Validate)]
pub struct RegisterUserSchema {
    pub name: String,
    pub password: String
}
