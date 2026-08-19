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
    #[validate(length(min = 1))]
    pub name: String,

    #[validate(length(min = 1))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct UpdateUserTargetWeightSchema {
    pub name: String,
    pub target_weight_kg: f32,
}

#[derive(Deserialize, Validate)]
pub struct UpdateUserStreakSchema {
    pub name: String,
    pub streak: i32,
}
