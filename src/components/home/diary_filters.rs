use super::date_range_dialog::DateRangeDialog;
use super::dates::short_date;
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::input::{Input, InputVariant};
use crate::components::ui::segmented::{Segmented, SegmentedOption};
use crate::schema::day::{DayFiltersSchema, DayOutcome};
use chrono::{Days, NaiveDate};
use dioxus::prelude::*;
use dioxus_icons::lucide::{Calendar, Plus};
use dioxus_sdk_time::use_debounce;
use std::time::Duration;

const SEARCH_DEBOUNCE: Duration = Duration::from_millis(300);

const SEGMENTED_MOBILE_CLASS: &str = "max-md:w-full max-md:*:flex-auto max-md:*:justify-center max-md:*:whitespace-nowrap max-md:*:px-2 max-md:*:py-[9px]";

#[derive(Clone, Copy, PartialEq, Default)]
pub enum DiaryRange {
    Week,
    #[default]
    Month,
    Custom {
        from: NaiveDate,
        to: NaiveDate,
    },
}

impl DiaryRange {
    /// The inclusive date bounds the server filters on.
    fn bounds(self, today: NaiveDate) -> (Option<NaiveDate>, Option<NaiveDate>) {
        match self {
            Self::Week => (Some(today - Days::new(6)), None),
            Self::Month => (Some(today - Days::new(29)), None),
            Self::Custom { from, to } => (Some(from), Some(to)),
        }
    }
}

#[derive(Clone, PartialEq, Default)]
pub struct DiaryFilters {
    /// `None` shows every day.
    pub outcome: Option<DayOutcome>,
    pub range: DiaryRange,
    pub query: String,
}

impl DiaryFilters {
    pub fn is_active(&self) -> bool {
        self.outcome.is_some() || self.range != DiaryRange::Month || !self.query.trim().is_empty()
    }

    pub fn day_filters(&self, today: NaiveDate) -> DayFiltersSchema {
        let (from, to) = self.range.bounds(today);
        DayFiltersSchema {
            from,
            to,
            outcome: self.outcome,
            q: Some(self.query.clone()).filter(|query| !query.is_empty()),
        }
    }
}

#[component]
pub fn DiaryFilterBar(
    filters: Signal<DiaryFilters>,
    today: NaiveDate,
    /// Without it the mobile "new entry" button is hidden.
    on_new_entry: Option<EventHandler>,
) -> Element {
    let mut is_date_dialog_open = use_signal(|| false);
    // The input updates on every keystroke, the diary query only once typing
    // pauses, so it doesn't hit the server per character.
    let mut search_input = use_signal(|| filters.peek().query.clone());
    let mut apply_search = use_debounce(SEARCH_DEBOUNCE, move |query: String| {
        filters.write().query = query;
    });
    let current = filters();

    let custom_range_label = match current.range {
        DiaryRange::Custom { from, to } => format!("{} – {}", short_date(from), short_date(to)),
        _ => "Date".to_string(),
    };

    rsx! {
        div {
            class: "flex flex-wrap items-center justify-between gap-4 border-b border-primary/15 pb-4 mb-4 max-md:items-stretch",
            div {
                class: "flex flex-wrap items-center gap-3",
                Segmented {
                    class: SEGMENTED_MOBILE_CLASS,
                    aria_label: "Filtra i giorni per esito",
                    SegmentedOption {
                        name: "diary-outcome",
                        checked: current.outcome.is_none(),
                        onclick: move |_| filters.write().outcome = None,
                        "Tutti"
                    }
                    SegmentedOption {
                        name: "diary-outcome",
                        checked: current.outcome == Some(DayOutcome::Within),
                        onclick: move |_| filters.write().outcome = Some(DayOutcome::Within),
                        "Entro obiettivo"
                    }
                    SegmentedOption {
                        name: "diary-outcome",
                        checked: current.outcome == Some(DayOutcome::Over),
                        onclick: move |_| filters.write().outcome = Some(DayOutcome::Over),
                        "Oltre obiettivo"
                    }
                }
                Segmented {
                    class: SEGMENTED_MOBILE_CLASS,
                    aria_label: "Filtra per periodo",
                    SegmentedOption {
                        name: "diary-range",
                        checked: current.range == DiaryRange::Week,
                        onclick: move |_| filters.write().range = DiaryRange::Week,
                        "7 giorni"
                    }
                    SegmentedOption {
                        name: "diary-range",
                        checked: current.range == DiaryRange::Month,
                        onclick: move |_| filters.write().range = DiaryRange::Month,
                        "Tutto il mese"
                    }
                    SegmentedOption {
                        name: "diary-range",
                        checked: matches!(current.range, DiaryRange::Custom { .. }),
                        // The range only becomes custom once the dialog is
                        // applied, so the radio must not check itself here.
                        onclick: move |e: MouseEvent| {
                            e.prevent_default();
                            is_date_dialog_open.set(true);
                        },
                        Calendar {
                            size: "13"
                        }
                        "{custom_range_label}"
                    }
                }
            }
            div {
                class: "flex flex-1 items-center gap-2 min-w-[180px] max-w-80",
                Input {
                    variant: InputVariant::Outline,
                    r#type: "search",
                    placeholder: "Cerca un alimento",
                    aria_label: "Cerca un alimento nel diario",
                    value: search_input(),
                    oninput: move |e: FormEvent| {
                        search_input.set(e.value());
                        apply_search.action(e.value());
                    },
                }
                if current.is_active() || !search_input().trim().is_empty() {
                    Button {
                        type: "button",
                        class: "flex-none text-accent text-[12.5px]",
                        variant: ButtonVariant::Ghost,
                        onclick: move |_| {
                            apply_search.cancel();
                            search_input.set(String::new());
                            filters.set(DiaryFilters::default());
                        },
                        "Azzera"
                    }
                }
                if let Some(on_new_entry) = on_new_entry {
                    Button {
                        type: "button",
                        class: "flex-none md:hidden",
                        variant: ButtonVariant::Primary,
                        size: ButtonSize::Icon,
                        aria_label: "Nuova voce",
                        onclick: move |_| on_new_entry.call(()),
                        Plus {
                            size: "1.25em"
                        }
                    }
                }
            }
        }
        DateRangeDialog {
            open: is_date_dialog_open,
            filters,
            today,
        }
    }
}
