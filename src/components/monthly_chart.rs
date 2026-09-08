use crate::components::ui::chart::{Chart, ChartSeries};
use crate::utils::constants::Month;
use chrono::{Datelike, Days, Duration, Local};
use dioxus::core::Element;
use dioxus::core_macro::{component, rsx, Props};
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct MonthlyChartProps {
    pub series: Vec<ChartSeries>,
}

#[component]
pub fn MonthlyChart(props: MonthlyChartProps) -> Element {
    let today = Local::now().date_naive();
    let first_day_of_current_month = today.with_day(1).expect("a date always has day 1");
    let last_day_of_previous_month = first_day_of_current_month - Days::new(1);

    let start_date = last_day_of_previous_month
        .with_day(today.day())
        .unwrap_or(first_day_of_current_month);

    let mut days = Vec::new();
    let mut day = start_date;
    while day <= today {
        days.push(format!(
            "{} {}",
            day.day(),
            Month::from_zero_based(day.month0()).short_name()
        ));
        day += Duration::days(1);
    }
    rsx! {
        Chart {
            days: days,
            series: props.series,
        }
    }
}
