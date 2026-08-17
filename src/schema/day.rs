use chrono::{DateTime, NaiveDate, Utc};

pub struct DaySchema {
    pub id: i32,
    pub date: NaiveDate,
    pub user_id: i32,
    pub weight_kg: f32,
    pub target_calories: i32,
    pub note: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
