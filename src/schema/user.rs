use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug)]
pub struct UserSchema {
    pub id: i32,
    pub name: String,
    pub streak: i32,
    pub target_weight_kg: Option<f32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Validate, Debug)]
pub struct RegisterUserSchema {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,

    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Deserialize, Serialize, Validate, Debug)]
pub struct LoginUserSchema {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,

    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct UpdateUserTargetWeightSchema {
    pub name: String,

    #[validate(range(min = 0.0))]
    pub target_weight_kg: f32,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct UpdateUserStreakSchema {
    pub name: String,

    #[validate(range(min = 0))]
    pub streak: i32,
}
