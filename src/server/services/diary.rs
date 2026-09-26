use crate::schema::day::DayFiltersSchema;
use crate::schema::diary::DiaryDaySchema;
use crate::schema::entry::{EntryFiltersSchema, EntrySchema};
use crate::server::error::ServerError;
use crate::server::repo::day::Day;
use crate::server::repo::entry::Entry;
use crate::server::services::{non_blank, users};
use sqlx::PgPool;

/// The days matching `filters`, newest first, each with all of its entries:
/// a day that matches the search is shown whole.
pub async fn list(
    user_name: &str,
    filters: DayFiltersSchema,
    pool: &PgPool,
) -> Result<Vec<DiaryDaySchema>, ServerError> {
    users::find(user_name, pool).await?;
    let filters = DayFiltersSchema {
        q: non_blank(filters.q),
        ..filters
    };
    let days = Day::find_by_user(user_name, &filters, pool).await?;

    let (Some(newest), Some(oldest)) = (days.first(), days.last()) else {
        return Ok(vec![]);
    };
    let entries: Vec<EntrySchema> = Entry::find_by_user(
        user_name,
        &EntryFiltersSchema {
            from: Some(oldest.date),
            to: Some(newest.date),
            q: None,
        },
        pool,
    )
    .await?
    .into_iter()
    .map(EntrySchema::from)
    .collect();

    Ok(days
        .into_iter()
        .map(|day| {
            let day_entries: Vec<EntrySchema> = entries
                .iter()
                .filter(|entry| entry.date == day.date)
                .cloned()
                .collect();
            DiaryDaySchema {
                calories: day_entries.iter().map(|entry| entry.calories).sum(),
                entries: day_entries,
                day: day.into(),
            }
        })
        .collect())
}
