use crate::{
    components::ui::badge::{Badge, BadgeVariant},
    utils::constants::SHORT_MONTHS,
};
use chrono::{Datelike, Days, Local, NaiveDate};
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct DayBlockProps {
    pub date: NaiveDate,
    pub weight_kg: Option<f32>,

    #[props(default = 0)]
    pub ingested_calories: i32,

    pub target_calories: i32,
    pub notes: Option<String>,
    pub children: Element,
}

#[component]
pub fn DayBlock(props: DayBlockProps) -> Element {
    let now = Local::now();
    let today = now.date_naive();

    let prefix = if today == props.date {
        "Oggi · "
    } else if today - Days::new(1) == props.date {
        "Ieri · "
    } else {
        ""
    };

    rsx! {
        div {
            class: "flex flex-col gap-2 mb-12",
            div {
                class: "flex items-center justify-between",
                p {
                    class: "font-heading text-xl",
                    "{prefix}{props.date.day()} {SHORT_MONTHS[props.date.month0() as usize]}"
                }
                Badge {
                    variant: BadgeVariant::Primary,
                    "{props.ingested_calories} kcal · obiettivo {props.target_calories}"
                }
            }
            {props.children}
        }
    }
}
