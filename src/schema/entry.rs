use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;
use validator::Validate;

pub struct EntrySchema {
    pub id: i32,
    pub date: NaiveDate,
    pub user_id: i32,
    pub name: String,
    pub calories: i32,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Validate)]
pub struct CreateEntrySchema {
    pub date: NaiveDate,
    pub user_id: i32,
    pub name: String,
    #[validate(range(min = 0))]
    pub calories: i32,
    pub notes: Option<String>,
}

#[derive(Deserialize)]
pub struct FindEntryByUserSchema {
    pub name: String,
}

#[derive(Deserialize)]
pub struct FindEntryByIdSchema {
    pub id: i32,
}

#[derive(Deserialize)]
pub struct UpdateEntryNotesSchema {
    pub id: i32,
    pub notes: Option<String>,
}

#[derive(Deserialize)]
pub struct DeleteEntrySchema {
    pub id: i32,
}
