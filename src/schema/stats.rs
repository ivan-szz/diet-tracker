use crate::schema::user::UserSchema;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Query string for a single user's summary, e.g. `?user_name=ivan&date=2026-09-26`.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct SummaryQuerySchema {
    /// Defaults to the signed-in user.
    pub user_name: Option<String>,
    /// The day the calorie progress refers to; defaults to the server's today.
    pub date: Option<NaiveDate>,
}

/// Query string for the community summary, e.g. `?date=2026-09-26`.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct CommunityQuerySchema {
    /// The day the calorie progress refers to; defaults to the server's today.
    pub date: Option<NaiveDate>,
}

/// Query string for a calorie/weight trend, e.g. `?from=2026-09-01&to=2026-09-26`.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct TrendQuerySchema {
    /// Defaults to the signed-in user.
    pub user_name: Option<String>,
    /// Inclusive; defaults to 29 days before `to`.
    pub from: Option<NaiveDate>,
    /// Inclusive; defaults to the server's today.
    pub to: Option<NaiveDate>,
}

/// Calories eaten on one date against the calorie target in force that day.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DayProgressSchema {
    pub date: NaiveDate,
    pub calories: i32,
    /// Carried over from the latest earlier day when `date` has no day of its own.
    pub target_calories: Option<i32>,
    /// Whether `date` has a day of its own.
    pub has_day: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct WeightProgressSchema {
    /// The most recently recorded weight.
    pub current_kg: Option<f32>,
    /// The first recorded weight and the date it was recorded on.
    pub starting_kg: Option<f32>,
    pub starting_date: Option<NaiveDate>,
    /// `current_kg - starting_kg`, or 0 until both exist.
    pub delta_kg: f32,
    pub target_kg: Option<f32>,
    /// How far from `starting_kg` to `target_kg` the user has come, in percent.
    pub goal_progress_percent: Option<f64>,
    /// Distance between `current_kg` and `target_kg`.
    pub remaining_kg: Option<f32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct UserSummarySchema {
    pub user: UserSchema,
    pub day: DayProgressSchema,
    pub weight: WeightProgressSchema,
    /// How many days the user has recorded in total.
    pub recorded_days: usize,
}

/// One date of a trend. Target and weight carry over from the latest earlier
/// day, since a date without a day just means the user didn't change them.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TrendPointSchema {
    pub date: NaiveDate,
    pub calories: i32,
    pub target_calories: Option<i32>,
    pub weight_kg: Option<f32>,
}
