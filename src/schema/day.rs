use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;
use validator::Validate;

pub struct DaySchema {
    pub id: i32,
    pub date: NaiveDate,
    pub user_id: i32,
    pub weight_kg: Option<f32>,
    pub target_calories: i32,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Validate)]
pub struct CreateDaySchema {
    pub date: NaiveDate,
    pub user_name: String,

    #[validate(range(min = 0.0))]
    pub weight_kg: Option<f32>,

    #[validate(range(min = 0))]
    pub target_calories: i32,

    pub notes: Option<String>,
}

#[derive(Deserialize)]
pub struct FindDaysByUserSchema {
    pub name: String,
}

#[derive(Deserialize)]
pub struct FindDayByUserSchema {
    pub user_name: String,
    pub date: NaiveDate,
}

#[derive(Deserialize, Validate)]
pub struct UpdateDayWeightSchema {
    pub user_name: String,
    pub date: NaiveDate,

    #[validate(range(min = 0.0))]
    pub weight_kg: Option<f32>,
}

#[derive(Deserialize, Validate)]
pub struct UpdateDayTargetCaloriesSchema {
    pub user_name: String,
    pub date: NaiveDate,

    #[validate(range(min = 0))]
    pub target_calories: i32,
}

#[derive(Deserialize)]
pub struct UpdateDayNotesSchema {
    pub user_name: String,
    pub date: NaiveDate,
    pub notes: Option<String>,
}
