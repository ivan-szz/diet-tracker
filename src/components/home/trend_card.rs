use crate::components::monthly_chart::MonthlyChart;
use crate::components::ui::card::Card;
use crate::components::ui::chart::ChartSeries;
use crate::schema::stats::TrendPointSchema;
use dioxus::prelude::*;

/// `points` must cover `MonthlyChart`'s date range, one per day, ascending.
/// `owner_name` is `None` for the signed-in user's own trend.
#[component]
pub fn TrendCard(points: Vec<TrendPointSchema>, owner_name: Option<String>) -> Element {
    let kicker = match owner_name {
        Some(name) => format!("L'ANDAMENTO DI {}", name.to_uppercase()),
        None => "IL TUO ANDAMENTO".to_string(),
    };
    let calories_series = points.iter().map(|point| point.calories as f64).collect();
    let target_calories_series = points
        .iter()
        .map(|point| point.target_calories.unwrap_or(0) as f64)
        .collect();
    let weight_series = points
        .iter()
        .map(|point| point.weight_kg.unwrap_or(0.0) as f64)
        .collect();

    rsx! {
        Card {
            p {
                class: "text-accent text-xs font-semibold",
                "{kicker}"
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
