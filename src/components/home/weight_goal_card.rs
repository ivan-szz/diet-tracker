use crate::components::ui::card::Card;
use crate::components::ui::progress::Progress;
use dioxus::prelude::*;
use dioxus_icons::lucide::ArrowRight;

#[component]
pub fn WeightGoalCard(
    starting_kg: f32,
    target_kg: f32,
    percent: f64,
    remaining_kg: f32,
) -> Element {
    rsx! {
        Card {
            p {
                class: "text-accent text-xs font-semibold",
                "OBIETTIVO PESO"
            }
            p {
                class: "font-heading text-xl mb-3",
                span {
                    class: "flex items-center gap-1",
                    "{starting_kg:.1} kg"
                    ArrowRight {}
                    "{target_kg:.1} kg"
                }
            }
            div {
                class: "relative w-full flex items-center",
                if percent > 20.0 {
                    p {
                        class: "absolute text-background font-heading text-sm md:text-base z-10 -translate-x-full pr-1.5 md:pr-3",
                        left: "{percent:.1}%",
                        "{percent:.1} %"
                    }
                }
                if percent <= 20.0 {
                    p {
                        class: "absolute text-primary font-heading z-10 left-1/2 -translate-x-1/2",
                        "{percent:.1} %"
                    }
                }
                div {
                    class: "w-full",
                    Progress {
                        value: percent,
                        max: 100
                    }
                }
            }
            div {
                class: "flex items-center justify-between mb-3",
                p {
                    class: "text-xs text-primary-light",
                    "Partenza: {starting_kg:.1} kg"
                }
                p {
                    class: "text-xs text-primary-light",
                    "Obiettivo: {target_kg:.1} kg"
                }
            }
            p {
                class: "text-sm text-primary-light",
                "Mancano {remaining_kg:.1} kg all'obiettivo"
            }
        }
    }
}
