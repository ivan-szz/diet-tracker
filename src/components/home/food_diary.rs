use super::feedback::{toast_error, toast_success};
use super::new_entry_dialog::NewEntryDialog;
use super::stats::calories_on;
use crate::api::entry;
use crate::components::ui::button::Button;
use crate::components::ui::card::Card;
use crate::components::ui::confirm_dialog::ConfirmDialog;
use crate::components::ui::separator::Separator;
use crate::components::{DayBlock, EntryRow};
use crate::schema::entry::DeleteEntrySchema;
use crate::schema::{day::DaySchema, entry::EntrySchema};
use crate::utils::error::error_message;
use chrono::NaiveDate;
use dioxus::prelude::*;
use dioxus_icons::lucide::Plus;
use dioxus_primitives::toast::use_toast;

#[component]
pub fn FoodDiary(
    user_name: String,
    today: NaiveDate,
    days: Vec<DaySchema>,
    entries: Vec<EntrySchema>,
    on_change: EventHandler,
) -> Element {
    let toast_api = use_toast();
    let mut is_new_entry_dialog_open = use_signal(|| false);
    let mut pending_delete_entry_id = use_signal(|| None::<i32>);
    let mut is_deleting_entry = use_signal(|| false);

    let delete_user_name = user_name.clone();
    let handle_delete_entry = move || {
        let user_name = delete_user_name.clone();
        async move {
            let Some(id) = pending_delete_entry_id() else {
                return;
            };

            is_deleting_entry.set(true);
            let result = entry::delete(DeleteEntrySchema { id, user_name }).await;
            is_deleting_entry.set(false);

            match result {
                Ok(()) => {
                    pending_delete_entry_id.set(None);
                    on_change.call(());
                    toast_success(toast_api, "Voce eliminata");
                }
                Err(error) => toast_error(toast_api, error_message(&error)),
            }
        }
    };

    rsx! {
        div {
            class: "flex justify-between items-center mt-10",
            h2 {
                class: "font-heading text-3xl",
                "Diario alimentare"
            }
            Button {
                type: "button",
                class: "font-heading text-xl",
                onclick: move |_| is_new_entry_dialog_open.set(true),
                Plus {
                    size: "2em"
                }
                "Nuova voce"
            }
            NewEntryDialog {
                open: is_new_entry_dialog_open,
                user_name,
                today,
                on_saved: on_change,
            }
        }
        Card {
            if days.is_empty() {
                p {
                    class: "text-sm text-primary-light",
                    "Nessun giorno registrato."
                }
            } else {
                div {
                    class: "space-y-4",
                    for (index, day) in days.iter().enumerate() {
                        Fragment {
                            key: "{day.id}",
                            if index > 0 {
                                Separator {
                                    class: "opacity-20"
                                }
                            }
                            DayBlock {
                                date: day.date,
                                weight_kg: day.weight_kg,
                                ingested_calories: calories_on(day.date, &entries),
                                target_calories: day.target_calories,
                                notes: day.notes.clone(),
                                for entry in entries.iter().filter(|entry| entry.date == day.date) {
                                    EntryRow {
                                        key: "{entry.id}",
                                        name: entry.name.clone(),
                                        calories: entry.calories,
                                        notes: entry.notes.clone().unwrap_or_default(),
                                        on_delete: {
                                            let id = entry.id;
                                            move |_| pending_delete_entry_id.set(Some(id))
                                        },
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        ConfirmDialog {
            open: pending_delete_entry_id().is_some(),
            on_open_change: move |open: bool| {
                if !open {
                    pending_delete_entry_id.set(None);
                }
            },
            title: "Eliminare questa voce?",
            description: "L'operazione non può essere annullata.",
            loading: is_deleting_entry(),
            on_confirm: move |_| handle_delete_entry(),
        }
    }
}
