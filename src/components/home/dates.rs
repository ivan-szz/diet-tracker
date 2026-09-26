use crate::utils::constants::Month;
use chrono::{Datelike, Days, NaiveDate};

/// First date of `MonthlyChart`'s own range, so the trend requested for it
/// lines up with the day axis the chart renders internally.
pub fn chart_start_date(today: NaiveDate) -> NaiveDate {
    let first_day_of_current_month = today.with_day(1).expect("a date always has day 1");
    let last_day_of_previous_month = first_day_of_current_month - Days::new(1);
    last_day_of_previous_month
        .with_day(today.day())
        .unwrap_or(first_day_of_current_month)
}

pub fn short_date(date: NaiveDate) -> String {
    format!(
        "{} {}",
        date.day(),
        Month::from_zero_based(date.month0()).short_name()
    )
}

pub fn month_name(date: NaiveDate) -> &'static str {
    Month::from_zero_based(date.month0()).full_name()
}
