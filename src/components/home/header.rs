use super::dates::month_name;
use super::target_calories_dialog::TargetCaloriesDialog;
use crate::api::auth::logout;
use crate::components::providers::auth::use_auth;
use crate::components::StreakIndicator;
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::card::Card;
use crate::components::ui::separator::Separator;
use crate::schema::stats::UserSummarySchema;
use crate::Route;
use chrono::{Datelike, NaiveDate};
use dioxus::prelude::*;
use dioxus_icons::lucide::{ArrowLeft, LogOut, Pencil};
use dioxus_primitives::toast::{use_toast, ToastOptions};
use std::time::Duration;

/// `is_me` is whether `summary` belongs to the signed-in user; anyone else's
/// header is read-only.
#[component]
pub fn HomeHeader(
    summary: UserSummarySchema,
    is_me: bool,
    today: NaiveDate,
    on_change: EventHandler,
) -> Element {
    let UserSummarySchema {
        user, day, weight, ..
    } = summary;
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

    let current_month = month_name(today);
    let year = today.year();

    let calories_value = format!(
        "{} / {}",
        day.calories,
        day.target_calories
            .map(|value| value.to_string())
            .unwrap_or_else(|| "–".to_string())
    );
    let calories_caption = "kcal oggi / obiettivo".to_string();

    let (weight_value, weight_caption) = match (weight.current_kg, weight.starting_date) {
        (Some(current_kg), Some(starting_date)) => (
            format!("{current_kg:.1} kg"),
            format!("{:.1} kg da {}", weight.delta_kg, month_name(starting_date)),
        ),
        _ => ("—".to_string(), "Nessun peso registrato".to_string()),
    };

    let streak = user.streak;
    let streak_at_risk = user.streak_at_risk;
    use_hook(|| {
        if is_me && streak_at_risk {
            toast_api.warning(
                "Serie in sospeso".to_string(),
                ToastOptions::new()
                    .description("Un giorno recente non è ancora segnato: puoi recuperarlo fino a due giorni dopo la sua data")
                    .duration(Duration::from_secs(20)),
            );
        }
    });

    rsx! {
        div {
            p {
                class: "text-accent text-xs font-semibold mb-2",
                "DIARIO ALIMENTARE · {current_month} {year}"
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
                        if is_me {
                            Button {
                                type: "button",
                                class: "mb-1",
                                variant: ButtonVariant::Primary,
                                onclick: move |_| handle_logout(),
                                LogOut {}
                            }
                        }
                    }
                    p {
                        class: "text-primary-light",
                        "Ultimi 30 giorni di monitoraggio"
                    }
                    if !is_me {
                        Link {
                            class: "inline-flex items-center gap-1 mt-2 text-sm text-accent hover:underline",
                            to: Route::Home {},
                            ArrowLeft {
                                size: "1em"
                            }
                            "Torna ai tuoi progressi"
                        }
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
                                StreakStat { streak, at_risk: streak_at_risk }
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
                            if is_me {
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
                    if is_me {
                        div {
                            class: "cursor-pointer",
                            role: "button",
                            onclick: move |_| is_target_calories_dialog_open.set(true),
                            Stat { value: calories_value, caption: calories_caption }
                        }
                    } else {
                        div {
                            Stat { value: calories_value, caption: calories_caption }
                        }
                    }
                    div {
                        Separator {
                            horizontal: false
                        }
                    }
                    div {
                        StreakStat { streak, at_risk: streak_at_risk }
                    }
                }
                if is_me {
                    TargetCaloriesDialog {
                        open: is_target_calories_dialog_open,
                        user_name: user.name.clone(),
                        today,
                        today_has_day: day.has_day,
                        current_target: day.target_calories,
                        on_saved: on_change,
                    }
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

#[component]
fn StreakStat(streak: i32, at_risk: bool) -> Element {
    rsx! {
        p {
            class: "font-heading text-2xl mb-1",
            StreakIndicator { streak, at_risk }
        }
        p {
            class: "text-xs text-primary-light",
            if at_risk { "serie in sospeso" } else { "giorni di fila" }
        }
    }
}
