use super::diary_filters::{DiaryFilterBar, DiaryFilters};
use super::feedback::{or_toast, toast_error, toast_success};
use super::new_entry_dialog::NewEntryDialog;
use crate::api::{diary, entry};
use crate::components::ui::button::Button;
use crate::components::ui::card::Card;
use crate::components::ui::confirm_dialog::ConfirmDialog;
use crate::components::ui::separator::Separator;
use crate::components::{DayBlock, EntryRow};
use crate::schema::day::DayQuerySchema;
use crate::schema::entry::DeleteEntrySchema;
use crate::utils::error::error_message;
use chrono::NaiveDate;
use dioxus::prelude::*;
use dioxus_icons::lucide::Plus;
use dioxus_primitives::toast::use_toast;

/// Shows `viewed_user_name`'s diary; it can only be edited when `is_me`, and
/// edits are made as `current_user_name`. `revision` changes whenever the
/// page's data does, so the diary refetches alongside the rest of the page.
#[component]
pub fn FoodDiary(
    viewed_user_name: String,
    is_me: bool,
    current_user_name: String,
    today: NaiveDate,
    total_days: usize,
    revision: ReadSignal<u32>,
    on_change: EventHandler,
) -> Element {
    let toast_api = use_toast();
    let mut is_new_entry_dialog_open = use_signal(|| false);
    let mut pending_delete_entry_id = use_signal(|| None::<i32>);
    let mut is_deleting_entry = use_signal(|| false);
    let filters = use_signal(DiaryFilters::default);

    let diary_user_name = viewed_user_name.clone();
    let diary_resource = use_resource(move || {
        revision();
        let current = filters();
        let user_name = diary_user_name.clone();

        async move {
            let query = DayQuerySchema {
                user_name: Some(user_name),
                filters: current.day_filters(today),
            };
            or_toast(diary::list(query).await, toast_api)
        }
    });

    let diary_state = diary_resource.read();
    let visible_days = diary_state.as_deref().unwrap_or_default();
    let count_label = if filters.read().is_active() {
        format!("{} di {} giorni", visible_days.len(), total_days)
    } else {
        format!("{} giorni registrati", total_days)
    };
    let title = if is_me {
        "Diario alimentare".to_string()
    } else {
        format!("Diario di {viewed_user_name}")
    };

    let delete_user_name = current_user_name.clone();
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
            class: "flex flex-wrap justify-between items-center gap-3 mt-10",
            h2 {
                class: "font-heading text-3xl",
                "{title}"
            }
            if is_me {
                Button {
                    type: "button",
                    // On mobile it moves next to the diary search, which only
                    // exists once there are days to filter.
                    class: if total_days == 0 { "font-heading text-xl" } else { "font-heading text-xl max-md:hidden" },
                    onclick: move |_| is_new_entry_dialog_open.set(true),
                    Plus {
                        size: "2em"
                    }
                    "Nuova voce"
                }
                NewEntryDialog {
                    open: is_new_entry_dialog_open,
                    user_name: current_user_name,
                    today,
                    on_saved: on_change,
                }
            }
        }
        Card {
            if total_days == 0 {
                p {
                    class: "text-sm text-primary-light",
                    "Nessun giorno registrato."
                }
            } else {
                DiaryFilterBar {
                    filters,
                    today,
                    on_new_entry: is_me.then_some(EventHandler::new(move |_| is_new_entry_dialog_open.set(true))),
                }
                p {
                    class: "text-xs text-primary-light mb-3",
                    "{count_label}"
                }
                if diary_state.is_some() && visible_days.is_empty() {
                    div {
                        class: "py-6 text-center",
                        p {
                            class: "font-heading text-lg mb-1.5",
                            "Nessun giorno corrisponde"
                        }
                        p {
                            class: "text-sm text-primary-light",
                            "Prova ad allargare il periodo o a cambiare la ricerca."
                        }
                    }
                }
                div {
                    class: "space-y-4",
                    for (index, diary_day) in visible_days.iter().enumerate() {
                        Fragment {
                            key: "{diary_day.day.id}",
                            if index > 0 {
                                Separator {
                                    class: "opacity-20"
                                }
                            }
                            DayBlock {
                                date: diary_day.day.date,
                                weight_kg: diary_day.day.weight_kg,
                                ingested_calories: diary_day.calories,
                                target_calories: diary_day.day.target_calories,
                                notes: diary_day.day.notes.clone(),
                                for entry in diary_day.entries.iter() {
                                    EntryRow {
                                        key: "{entry.id}",
                                        name: entry.name.clone(),
                                        calories: entry.calories,
                                        notes: entry.notes.clone().unwrap_or_default(),
                                        on_delete: is_me.then(|| {
                                            let id = entry.id;
                                            EventHandler::new(move |_| pending_delete_entry_id.set(Some(id)))
                                        }),
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
