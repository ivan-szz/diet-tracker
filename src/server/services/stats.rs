use crate::schema::day::DayFiltersSchema;
use crate::schema::entry::EntryFiltersSchema;
use crate::schema::stats::{
    DayProgressSchema, TrendPointSchema, UserSummarySchema, WeightProgressSchema,
};
use crate::schema::user::UserSchema;
use crate::server::error::ServerError;
use crate::server::repo::day::Day;
use crate::server::repo::entry::Entry;
use crate::server::repo::user::User;
use crate::server::services::users;
use chrono::{Days, NaiveDate};
use sqlx::PgPool;

/// The longest trend served in one request.
const MAX_TREND_DAYS: u64 = 366;

pub async fn summary(
    user_name: &str,
    date: NaiveDate,
    pool: &PgPool,
) -> Result<UserSummarySchema, ServerError> {
    let user = users::find(user_name, pool).await?;

    summarize(user, date, pool).await
}

pub async fn community(
    date: NaiveDate,
    pool: &PgPool,
) -> Result<Vec<UserSummarySchema>, ServerError> {
    let users = User::find_all(pool).await?;

    let mut summaries = Vec::with_capacity(users.len());
    for user in users {
        summaries.push(summarize(user.into(), date, pool).await?);
    }

    Ok(summaries)
}

/// One point per date in `from..=to`, ascending.
pub async fn trend(
    user_name: &str,
    from: NaiveDate,
    to: NaiveDate,
    pool: &PgPool,
) -> Result<Vec<TrendPointSchema>, ServerError> {
    if from > to || to - Days::new(MAX_TREND_DAYS) > from {
        return Err(ServerError::InvalidDateRange);
    }
    users::find(user_name, pool).await?;

    // Earlier days are needed too: target and weight carry over from them.
    let days = Day::find_by_user(
        user_name,
        &DayFiltersSchema {
            to: Some(to),
            ..Default::default()
        },
        pool,
    )
    .await?;
    let entries = Entry::find_by_user(
        user_name,
        &EntryFiltersSchema {
            from: Some(from),
            to: Some(to),
            q: None,
        },
        pool,
    )
    .await?;

    let mut points = Vec::new();
    let mut date = from;
    while date <= to {
        let latest_day = days.iter().find(|day| day.date <= date);
        points.push(TrendPointSchema {
            date,
            calories: calories_on(date, &entries),
            target_calories: latest_day.map(|day| day.target_calories),
            weight_kg: days
                .iter()
                .filter(|day| day.date <= date)
                .find_map(|day| day.weight_kg),
        });
        date = date + Days::new(1);
    }

    Ok(points)
}

async fn summarize(
    user: UserSchema,
    date: NaiveDate,
    pool: &PgPool,
) -> Result<UserSummarySchema, ServerError> {
    let days = Day::find_by_user(&user.name, &DayFiltersSchema::default(), pool).await?;
    let entries = Entry::find_by_user(
        &user.name,
        &EntryFiltersSchema {
            from: Some(date),
            to: Some(date),
            q: None,
        },
        pool,
    )
    .await?;

    Ok(UserSummarySchema {
        day: day_progress(date, &days, &entries),
        weight: weight_progress(&days, user.target_weight_kg),
        recorded_days: days.len(),
        user,
    })
}

/// `days` must be sorted newest first, as the repository returns them.
fn day_progress(date: NaiveDate, days: &[Day], entries: &[Entry]) -> DayProgressSchema {
    DayProgressSchema {
        date,
        calories: calories_on(date, entries),
        target_calories: days
            .iter()
            .find(|day| day.date <= date)
            .map(|day| day.target_calories),
        has_day: days.iter().any(|day| day.date == date),
    }
}

/// `days` must be sorted newest first, as the repository returns them.
fn weight_progress(days: &[Day], target_kg: Option<f32>) -> WeightProgressSchema {
    let current_kg = days.iter().find_map(|day| day.weight_kg);
    let starting = days
        .iter()
        .rev()
        .find_map(|day| day.weight_kg.map(|kg| (kg, day.date)));
    let starting_kg = starting.map(|(kg, _)| kg);

    let goal_progress_percent = match (starting_kg, current_kg, target_kg) {
        (Some(starting_kg), Some(current_kg), Some(target_kg)) if starting_kg != target_kg => {
            let remaining_share = (current_kg - target_kg).abs() / (starting_kg - target_kg).abs();
            Some(100.0 - 100.0 * f64::from(remaining_share))
        }
        _ => None,
    };

    WeightProgressSchema {
        current_kg,
        starting_kg,
        starting_date: starting.map(|(_, date)| date),
        delta_kg: match (current_kg, starting_kg) {
            (Some(current_kg), Some(starting_kg)) => current_kg - starting_kg,
            _ => 0.0,
        },
        target_kg,
        goal_progress_percent,
        remaining_kg: current_kg
            .zip(target_kg)
            .map(|(current_kg, target_kg)| (current_kg - target_kg).abs()),
    }
}

fn calories_on(date: NaiveDate, entries: &[Entry]) -> i32 {
    entries
        .iter()
        .filter(|entry| entry.date == date)
        .map(|entry| entry.calories)
        .sum()
}
