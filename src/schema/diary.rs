use crate::schema::{day::DaySchema, entry::EntrySchema};
use serde::{Deserialize, Serialize};

/// A day of the food diary with its entries and their calorie total.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DiaryDaySchema {
    pub day: DaySchema,
    pub calories: i32,
    pub entries: Vec<EntrySchema>,
}
