use super::dates::short_date;
use super::feedback::{parse_form, toast_error, toast_success};
use crate::api::entry;
use crate::components::ui::accordion::{
    Accordion, AccordionContent, AccordionItem, AccordionTrigger,
};
use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::dialog::{Dialog, DialogDescription, DialogTitle};
use crate::components::ui::input::Input;
use crate::components::ui::label::Label;
use crate::schema::entry::CreateEntrySchema;
use crate::utils::error::error_message;
use chrono::NaiveDate;
use dioxus::prelude::*;
use dioxus_primitives::toast::use_toast;

#[component]
pub fn NewEntryDialog(
    open: Signal<bool>,
    user_name: String,
    today: NaiveDate,
    on_saved: EventHandler,
) -> Element {
    rsx! {
        Dialog {
            open: open(),
            on_open_change: move |v| open.set(v),
            Fragment {
                key: "{open()}",
                DialogTitle {
                    "Aggiungi una voce"
                }
                DialogDescription {
                    "Registra un alimento nel diario."
                }
                NewEntryForm { open, user_name, today, on_saved }
            }
        }
    }
}

/// Keyed on the dialog's open state by its parent, so the chosen date goes
/// back to today every time the dialog opens.
#[component]
fn NewEntryForm(
    open: Signal<bool>,
    user_name: String,
    today: NaiveDate,
    on_saved: EventHandler,
) -> Element {
    let toast_api = use_toast();
    let mut is_saving = use_signal(|| false);
    // Lives outside the accordion because its content unmounts when closed.
    let mut entry_date = use_signal(|| today);

    let entry_date_label = if entry_date() == today {
        "oggi".to_string()
    } else {
        short_date(entry_date())
    };

    let handle_create = move |e: Event<FormData>| async move {
        e.prevent_default();
        let Some(mut payload) = parse_form::<CreateEntrySchema>(&e, toast_api) else {
            return;
        };
        // An empty "Note" input round-trips as `Some("")`, not `None`.
        payload.notes = payload
            .notes
            .map(|notes| notes.trim().to_string())
            .filter(|notes| !notes.is_empty());

        is_saving.set(true);
        let result = entry::create(payload).await;
        is_saving.set(false);

        match result {
            Ok(_) => {
                open.set(false);
                on_saved.call(());
                toast_success(toast_api, "Voce aggiunta");
            }
            Err(error) => toast_error(toast_api, error_message(&error)),
        }
    };

    rsx! {
        form {
            onsubmit: move |e| handle_create(e),
            input { r#type: "hidden", name: "user_name", value: "{user_name}" }
            input { r#type: "hidden", name: "date", value: "{entry_date}" }
            div {
                class: "space-y-2 mb-4",
                Label {
                    html_for: "entry_name",
                    "Cosa hai mangiato?"
                }
                Input {
                    id: "entry_name",
                    name: "name",
                }
            }
            div {
                class: "space-y-2 mb-4",
                Label {
                    html_for: "entry_calories",
                    "Calorie (kcal)"
                }
                Input {
                    id: "entry_calories",
                    name: "calories",
                    type: "number",
                    min: "0",
                }
            }
            div {
                class: "space-y-2 mb-4",
                Label {
                    html_for: "entry_notes",
                    "Note (opzionale)"
                }
                Input {
                    id: "entry_notes",
                    name: "notes",
                }
            }
            div {
                class: "mb-6",
                Accordion {
                    AccordionItem {
                        index: 0,
                        AccordionTrigger {
                            span {
                                class: "text-sm py-2",
                                "Giorno: {entry_date_label}"
                            }
                        }
                        AccordionContent {
                            div {
                                class: "space-y-2 pt-2 pb-1",
                                Label {
                                    html_for: "entry_date",
                                    "Registra la voce in un altro giorno"
                                }
                                Input {
                                    id: "entry_date",
                                    type: "date",
                                    value: "{entry_date}",
                                    oninput: move |e: FormEvent| {
                                        if let Ok(date) = e.value().parse() {
                                            entry_date.set(date);
                                        }
                                    },
                                }
                            }
                        }
                    }
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
