use super::feedback::{parse_form, toast_error, toast_success};
use crate::api::day;
use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::dialog::{Dialog, DialogTitle};
use crate::components::ui::input::Input;
use crate::components::ui::label::Label;
use crate::schema::day::{CreateDaySchema, UpdateDayTargetCaloriesSchema};
use crate::utils::error::error_message;
use chrono::NaiveDate;
use dioxus::prelude::*;
use dioxus_primitives::toast::use_toast;

#[component]
pub fn TargetCaloriesDialog(
    open: Signal<bool>,
    user_name: String,
    today: NaiveDate,
    today_has_day: bool,
    current_target: Option<i32>,
    on_saved: EventHandler,
) -> Element {
    let toast_api = use_toast();
    let mut is_saving = use_signal(|| false);

    let handle_save = move |e: Event<FormData>| async move {
        e.prevent_default();
        let Some(payload) = parse_form::<CreateDaySchema>(&e, toast_api) else {
            return;
        };

        is_saving.set(true);

        let result = if today_has_day {
            day::update_target_calories(UpdateDayTargetCaloriesSchema {
                user_name: payload.user_name,
                date: payload.date,
                target_calories: payload.target_calories.unwrap_or_default(),
            })
            .await
            .map(|_| ())
        } else {
            day::create(payload).await.map(|_| ())
        };

        is_saving.set(false);

        match result {
            Ok(()) => {
                open.set(false);
                on_saved.call(());
                toast_success(toast_api, "Obiettivo calorico aggiornato");
            }
            Err(error) => toast_error(toast_api, error_message(&error)),
        }
    };

    rsx! {
        Dialog {
            open: open(),
            on_open_change: move |v| open.set(v),
            Fragment {
                key: "{open()}",
                DialogTitle {
                    "Aggiorna l'obiettivo calorico"
                }
                form {
                    onsubmit: move |e| handle_save(e),
                    input { r#type: "hidden", name: "user_name", value: "{user_name}" }
                    input { r#type: "hidden", name: "date", value: "{today}" }
                    div {
                        class: "space-y-2 mb-6",
                        Label {
                            html_for: "target_calories",
                            "Nuovo obiettivo (kcal/giorno)"
                        }
                        Input {
                            id: "target_calories",
                            name: "target_calories",
                            type: "number",
                            min: "0",
                            value: current_target.map(|value| value.to_string()).unwrap_or_default(),
                        }
                    }
                    div {
                        class: "flex justify-end items-center gap-4",
                        Button {
                            type: "button",
                            onclick: move |_| open.set(false),
                            variant: ButtonVariant::Outline,
                            disabled: is_saving(),
                            "Annulla"
                        }
                        Button {
                            type: "submit",
                            variant: ButtonVariant::Primary,
                            disabled: is_saving(),
                            "Salva"
                        }
                    }
                }
            }
        }
    }
}
