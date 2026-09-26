use super::diary_filters::{DiaryFilters, DiaryRange};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::dialog::{Dialog, DialogTitle};
use crate::components::ui::input::Input;
use crate::components::ui::label::Label;
use chrono::{Days, NaiveDate};
use dioxus::prelude::*;

const PRESETS: [(&str, u64); 3] = [
    ("Ultimi 7 giorni", 7),
    ("Ultimi 14 giorni", 14),
    ("Ultimi 30 giorni", 30),
];

#[component]
pub fn DateRangeDialog(
    open: Signal<bool>,
    filters: Signal<DiaryFilters>,
    today: NaiveDate,
) -> Element {
    rsx! {
        Dialog {
            open: open(),
            on_open_change: move |v| open.set(v),
            Fragment {
                key: "{open()}",
                DialogTitle {
                    "Scegli un intervallo di date"
                }
                DateRangeForm { open, filters, today }
            }
        }
    }
}

/// Keyed on the dialog's open state by its parent, so the draft restarts from
/// the applied range every time the dialog opens.
#[component]
fn DateRangeForm(open: Signal<bool>, filters: Signal<DiaryFilters>, today: NaiveDate) -> Element {
    let (initial_from, initial_to) = match filters.peek().range {
        DiaryRange::Custom { from, to } => (from, to),
        _ => (today - Days::new(6), today),
    };
    let mut draft_from = use_signal(|| Some(initial_from));
    let mut draft_to = use_signal(|| Some(initial_to));

    let is_inverted = matches!((draft_from(), draft_to()), (Some(from), Some(to)) if from > to);
    let can_apply = draft_from().is_some() && draft_to().is_some() && !is_inverted;

    let apply = move |_| {
        if let (Some(from), Some(to)) = (draft_from(), draft_to()) {
            filters.write().range = DiaryRange::Custom { from, to };
            open.set(false);
        }
    };

    rsx! {
        div {
            class: "flex flex-wrap gap-3 mb-3",
            div {
                class: "space-y-2 flex-1 min-w-[150px]",
                Label {
                    html_for: "date_from",
                    "Dal giorno"
                }
                Input {
                    id: "date_from",
                    r#type: "date",
                    max: "{today}",
                    value: draft_from().map(|date| date.to_string()).unwrap_or_default(),
                    oninput: move |e: FormEvent| draft_from.set(e.value().parse().ok()),
                }
            }
            div {
                class: "space-y-2 flex-1 min-w-[150px]",
                Label {
                    html_for: "date_to",
                    "Al giorno"
                }
                Input {
                    id: "date_to",
                    r#type: "date",
                    max: "{today}",
                    value: draft_to().map(|date| date.to_string()).unwrap_or_default(),
                    oninput: move |e: FormEvent| draft_to.set(e.value().parse().ok()),
                }
            }
        }
        div {
            class: "flex flex-wrap gap-2 mb-3",
            for (label, days) in PRESETS {
                Button {
                    key: "{days}",
                    type: "button",
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        draft_from.set(Some(today - Days::new(days - 1)));
                        draft_to.set(Some(today));
                    },
                    "{label}"
                }
            }
        }
        if is_inverted {
            p {
                class: "text-sm text-accent mb-3",
                "La data iniziale deve precedere quella finale."
            }
        }
        div {
            class: "flex justify-end items-center gap-4 mt-3",
            Button {
                type: "button",
                variant: ButtonVariant::Outline,
                onclick: move |_| open.set(false),
                "Annulla"
            }
            Button {
                type: "button",
                variant: ButtonVariant::Primary,
                disabled: !can_apply,
                onclick: apply,
                "Applica"
            }
        }
    }
}
