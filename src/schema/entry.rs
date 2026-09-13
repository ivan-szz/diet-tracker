use crate::utils::serde::number_from_string;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug)]
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

#[derive(Deserialize, Serialize, Validate)]
pub struct CreateEntrySchema {
    pub date: NaiveDate,
    pub user_name: String,
    #[validate(length(min = 1, message = "Inserisci un nome per la voce"))]
    pub name: String,
    #[serde(deserialize_with = "number_from_string")]
    #[validate(range(min = 0, message = "Le calorie non possono essere negative"))]
    pub calories: i32,
    pub notes: Option<String>,
}

#[derive(Deserialize)]
pub struct FindEntriesByUserSchema {
    pub name: String,
}

#[derive(Deserialize)]
pub struct FindEntryByIdSchema {
    pub id: i32,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateEntryNotesSchema {
    pub id: i32,
    pub user_name: String,
    pub notes: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct DeleteEntrySchema {
    pub id: i32,
    pub user_name: String,
}
