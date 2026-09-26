use super::stats::{calories_on, first_weight, latest_weight, target_calories_as_of};
use super::target_calories_dialog::TargetCaloriesDialog;
use crate::api::auth::logout;
use crate::components::providers::auth::use_auth;
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::card::Card;
use crate::components::ui::separator::Separator;
use crate::schema::{day::DaySchema, entry::EntrySchema, user::UserSchema};
use crate::utils::constants::Month;
use chrono::{Datelike, NaiveDate};
use dioxus::prelude::*;
use dioxus_icons::lucide::{LogOut, Pencil};
use dioxus_primitives::toast::{use_toast, ToastOptions};
use std::time::Duration;

#[component]
pub fn HomeHeader(
    user: UserSchema,
    days: Vec<DaySchema>,
    entries: Vec<EntrySchema>,
    today: NaiveDate,
    on_change: EventHandler,
) -> Element {
    let session = use_auth();
    let toast_api = use_toast();
    let navigator = use_navigator();
    let mut is_target_calories_dialog_open = use_signal(|| false);

    let handle_logout = move || async move {
        let _ = logout().await;
        session.clear();
        toast_api.info(
            "Disconnesso".to_string(),
            ToastOptions::new()
                .description("Ti sei disconnesso")
                .duration(Duration::from_secs(20)),
        );
        navigator.push("/login");
    };

    let month = Month::from_zero_based(today.month0());
    let year = today.year();

    let today_target_calories = target_calories_as_of(today, &days);
    let calories_value = format!(
        "{} / {}",
        calories_on(today, &entries),
        today_target_calories
            .map(|value| value.to_string())
            .unwrap_or_else(|| "–".to_string())
    );
    let calories_caption = "kcal oggi / obiettivo".to_string();

    let (weight_value, weight_caption) = match (latest_weight(&days), first_weight(&days)) {
        (Some(current_kg), Some((starting_kg, starting_date))) => (
            format!("{current_kg:.1} kg"),
            format!(
                "{:.1} kg da {}",
                current_kg - starting_kg,
                Month::from_zero_based(starting_date.month0()).full_name()
            ),
        ),
        _ => ("—".to_string(), "Nessun peso registrato".to_string()),
    };

    let streak_value = user.streak.to_string();
    let streak_caption = "giorni di fila".to_string();

    rsx! {
        div {
            p {
                class: "text-accent text-xs font-semibold mb-2",
                "DIARIO ALIMENTARE · {month.full_name()} {year}"
            }
            div {
                class: "flex flex-col gap-6 md:flex-row md:justify-between md:items-end",
                div {
                    div {
                        class: "flex items-end gap-4 mb-3",
                        h1 {
                            class: "font-heading text-4xl md:text-5xl",
                            "{user.name}"
                        }
                        Button {
                            type: "button",
                            class: "mb-1",
                            variant: ButtonVariant::Primary,
                            onclick: move |_| handle_logout(),
                            LogOut {}
                        }
                    }
                    p {
                        class: "text-primary-light",
                        "Ultimi 30 giorni di monitoraggio"
                    }
                }
                div {
                    class: "md:hidden",
                    Card {
                        div {
                            class: "grid grid-cols-2 gap-y-4",
                            div {
                                Stat { value: weight_value.clone(), caption: weight_caption.clone() }
                            }
                            div {
                                class: "text-right",
                                Stat { value: streak_value.clone(), caption: streak_caption.clone() }
                            }
                            div {
                                class: "col-span-2",
                                Separator {
                                    horizontal: true
                                }
                            }
                            div {
                                Stat { value: calories_value.clone(), caption: calories_caption.clone() }
                            }
                            div {
                                class: "flex items-center justify-end",
                                Button {
                                    type: "button",
                                    variant: ButtonVariant::Outline,
                                    size: ButtonSize::Sm,
                                    onclick: move |_| is_target_calories_dialog_open.set(true),
                                    Pencil {}
                                    "Obiettivo"
                                }
                            }
                        }
                    }
                }
                div {
                    class: "hidden md:flex md:gap-8 content-center",
                    div {
                        Stat { value: weight_value, caption: weight_caption }
                    }
                    div {
                        Separator {
                            horizontal: false
                        }
                    }
                    div {
                        class: "cursor-pointer",
                        role: "button",
                        onclick: move |_| is_target_calories_dialog_open.set(true),
                        Stat { value: calories_value, caption: calories_caption }
                    }
                    div {
                        Separator {
                            horizontal: false
                        }
                    }
                    div {
                        Stat { value: streak_value, caption: streak_caption }
                    }
                }
                TargetCaloriesDialog {
                    open: is_target_calories_dialog_open,
                    user_name: user.name.clone(),
                    today,
                    today_has_day: days.iter().any(|day| day.date == today),
                    current_target: today_target_calories,
                    on_saved: on_change,
                }
            }
        }
    }
}

#[component]
fn Stat(value: String, caption: String) -> Element {
    rsx! {
        p {
            class: "font-heading text-2xl mb-1",
            "{value}"
        }
        p {
            class: "text-xs text-primary-light",
            "{caption}"
        }
    }
}
