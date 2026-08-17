use chrono::{DateTime, NaiveDate, Utc};

pub struct EntrySchema {
    pub id: i32,
    pub date: NaiveDate,
    pub user_id: i32,
    pub name: String,
    pub calories: i32,
    pub notes: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
