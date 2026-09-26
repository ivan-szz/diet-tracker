use super::stats::{trailing_month_dates, trailing_trend};
use crate::components::monthly_chart::MonthlyChart;
use crate::components::ui::card::Card;
use crate::components::ui::chart::ChartSeries;
use crate::schema::{day::DaySchema, entry::EntrySchema};
use chrono::NaiveDate;
use dioxus::prelude::*;

#[component]
pub fn TrendCard(days: Vec<DaySchema>, entries: Vec<EntrySchema>, today: NaiveDate) -> Element {
    let chart_dates = trailing_month_dates(today);
    let (calories_series, target_calories_series, weight_series) =
        trailing_trend(&chart_dates, &days, &entries);

    rsx! {
        Card {
            p {
                class: "text-accent text-xs font-semibold",
                // TODO: Seguirà l'utente selezionato, una volta che esisterà.
                "IL TUO ANDAMENTO"
            }
            p {
                class: "font-heading text-xl",
                "Ultimi 30 giorni"
            }
            p {
                class: "text-sm text-primary-light mb-6",
                "Passa il cursore sul grafico per confrontare calorie e peso di un singolo giorno."
            }
            MonthlyChart {
                series: vec![
                    ChartSeries::new("Kcal assunte", " kcal", calories_series).with_floor(0.0),
                    ChartSeries::new("Obiettivo kcal", " kcal", target_calories_series)
                        .with_color("#6B665E")
                        .dashed(),
                    ChartSeries::new("Peso", " kg", weight_series)
                        .with_decimals(1)
                        .with_floor(0.0),
                ],
            }
        }
    }
}
