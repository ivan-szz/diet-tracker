use crate::utils::serde::{number_from_string, optional_number_from_string};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug)]
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

#[derive(Deserialize, Serialize, Validate)]
pub struct CreateDaySchema {
    pub date: NaiveDate,
    pub user_name: String,

    #[validate(range(min = 0.0))]
    pub weight_kg: Option<f32>,

    /// Defaults to the previous day's target when omitted.
    #[serde(default, deserialize_with = "optional_number_from_string")]
    #[validate(range(min = 0, message = "Le calorie non possono essere negative"))]
    pub target_calories: Option<i32>,

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

#[derive(Deserialize, Serialize, Validate)]
pub struct UpdateDayWeightSchema {
    pub user_name: String,
    pub date: NaiveDate,

    #[validate(range(min = 0.0))]
    pub weight_kg: Option<f32>,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct UpdateDayTargetCaloriesSchema {
    pub user_name: String,
    pub date: NaiveDate,

    #[serde(deserialize_with = "number_from_string")]
    #[validate(range(min = 0, message = "Le calorie non possono essere negative"))]
    pub target_calories: i32,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateDayNotesSchema {
    pub user_name: String,
    pub date: NaiveDate,
    pub notes: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct DeleteDaySchema {
    pub user_name: String,
    pub date: NaiveDate,
}
