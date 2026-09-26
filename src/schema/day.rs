use crate::utils::serde::{number_from_string, optional_number_from_string};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
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

/// How a day's eaten calories compare to its calorie target.
#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DayOutcome {
    /// Eaten calories are at most the target.
    Within,
    Over,
}

impl DayOutcome {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Within => "within",
            Self::Over => "over",
        }
    }
}

/// Query string filters for listing days, e.g.
/// `?from=2026-09-01&to=2026-09-30&outcome=over&q=pasta`. Omitted fields don't filter.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct DayFiltersSchema {
    /// Inclusive lower bound on the day's date.
    pub from: Option<NaiveDate>,
    /// Inclusive upper bound on the day's date.
    pub to: Option<NaiveDate>,
    pub outcome: Option<DayOutcome>,
    /// Case-insensitive text that at least one of the day's entries must
    /// contain in its name or notes.
    pub q: Option<String>,
}

/// Query string for listing a user's days: whose, plus [`DayFiltersSchema`].
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct DayQuerySchema {
    /// Defaults to the signed-in user.
    pub user_name: Option<String>,
    #[serde(flatten)]
    pub filters: DayFiltersSchema,
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
