use crate::schema::{day::DaySchema, entry::EntrySchema};
use crate::utils::constants::Month;
use chrono::{Datelike, Days, NaiveDate};

/// Mirrors `MonthlyChart`'s own date range exactly, so the series built here
/// line up with the day axis it renders internally.
pub fn trailing_month_dates(today: NaiveDate) -> Vec<NaiveDate> {
    let first_day_of_current_month = today.with_day(1).expect("a date always has day 1");
    let last_day_of_previous_month = first_day_of_current_month - Days::new(1);
    let start_date = last_day_of_previous_month
        .with_day(today.day())
        .unwrap_or(first_day_of_current_month);

    let mut dates = Vec::new();
    let mut date = start_date;
    while date <= today {
        dates.push(date);
        date = date + Days::new(1);
    }
    dates
}

/// One value per day in `dates`, ascending. A day with no entries shows 0
/// calories; target calories and weight forward-fill from the last known day,
/// since the user simply didn't touch those on a day without a `Day` row.
pub fn trailing_trend(
    dates: &[NaiveDate],
    days: &[DaySchema],
    entries: &[EntrySchema],
) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let mut sorted_days: Vec<&DaySchema> = days.iter().collect();
    sorted_days.sort_by_key(|day| day.date);
    let mut day_cursor = sorted_days.into_iter().peekable();

    let mut calories_series = Vec::with_capacity(dates.len());
    let mut target_series = Vec::with_capacity(dates.len());
    let mut weight_series = Vec::with_capacity(dates.len());

    let mut last_target: Option<i32> = None;
    let mut last_weight: Option<f32> = None;

    for &date in dates {
        while day_cursor.peek().is_some_and(|day| day.date <= date) {
            let day = day_cursor.next().expect("peeked Some above");
            last_target = Some(day.target_calories);
            last_weight = day.weight_kg.or(last_weight);
        }

        target_series.push(last_target.unwrap_or(0) as f64);
        weight_series.push(last_weight.unwrap_or(0.0) as f64);
        calories_series.push(calories_on(date, entries) as f64);
    }

    (calories_series, target_series, weight_series)
}

pub fn calories_on(date: NaiveDate, entries: &[EntrySchema]) -> i32 {
    entries
        .iter()
        .filter(|entry| entry.date == date)
        .map(|entry| entry.calories)
        .sum()
}

pub fn target_calories_as_of(date: NaiveDate, days: &[DaySchema]) -> Option<i32> {
    days.iter()
        .find(|day| day.date <= date)
        .map(|day| day.target_calories)
}

/// Calories eaten and calorie target for `date`, from one user's `days`/`entries`.
pub fn day_progress(date: NaiveDate, days: &[DaySchema], entries: &[EntrySchema]) -> (i32, i32) {
    let target_calories = target_calories_as_of(date, days).unwrap_or(0);
    (calories_on(date, entries), target_calories)
}

pub fn latest_weight(days: &[DaySchema]) -> Option<f32> {
    days.iter().find_map(|day| day.weight_kg)
}

/// The earliest recorded weight and the date it was recorded on.
pub fn first_weight(days: &[DaySchema]) -> Option<(f32, NaiveDate)> {
    days.iter()
        .rev()
        .find_map(|day| day.weight_kg.map(|kg| (kg, day.date)))
}

pub fn weight_delta(days: &[DaySchema]) -> f32 {
    match (latest_weight(days), first_weight(days)) {
        (Some(latest_kg), Some((first_kg, _))) => latest_kg - first_kg,
        _ => 0.0,
    }
}

pub fn weight_starting_month(days: &[DaySchema]) -> Option<String> {
    first_weight(days).map(|(_, date)| {
        Month::from_zero_based(date.month0())
            .full_name()
            .to_string()
    })
}
