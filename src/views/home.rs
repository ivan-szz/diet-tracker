use crate::api::{auth::logout, day, entry, users};
use crate::components::monthly_chart::MonthlyChart;
use crate::components::providers::auth::use_auth;
use crate::components::ui::button::{ButtonSize, ButtonVariant};
use crate::components::ui::dialog::{Dialog, DialogDescription, DialogTitle};
use crate::components::ui::input::Input;
use crate::components::ui::label::Label;
use crate::schema::day::{CreateDaySchema, UpdateDayTargetCaloriesSchema};
use crate::schema::{day::DaySchema, entry::EntrySchema, user::UserSchema};
use crate::utils::error::error_message;
use crate::{
    components::{
        ui::{
            accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger},
            button::Button,
            card::Card,
            chart::ChartSeries,
            progress::Progress,
            separator::Separator,
        },
        DayBlock, EntryRow, UserRow,
    },
    utils::constants::Month,
};
use chrono::{Datelike, Days, Local, NaiveDate};
use dioxus::prelude::*;
use dioxus_html::a::size;
use dioxus_icons::lucide::{ArrowRight, LogOut, Pencil, Plus};
use dioxus_primitives::toast::{use_toast, ToastOptions};
use std::time::Duration;

struct HomeData {
    community: Vec<CommunityMember>,
    days: Vec<DaySchema>,
    entries: Vec<EntrySchema>,
}

/// A community member paired with the stats `UserRow` shows for them. Their
/// `days`/`entries` are public read data, fetched the same way regardless of
/// whether they belong to the signed-in user.
struct CommunityMember {
    user: UserSchema,
    weight_delta: f32,
    calories: i32,
    target_calories: i32,
}

/// Mirrors `MonthlyChart`'s own date range exactly, so the series built here
/// line up with the day axis it renders internally.
fn trailing_month_dates(today: NaiveDate) -> Vec<NaiveDate> {
    let first_day_of_current_month = today.with_day(1).expect("a date always has day 1");
    let last_day_of_previous_month = first_day_of_current_month - Days::new(1);
    let start_date = last_day_of_previous_month
        .with_day(today.day())
        .unwrap_or(first_day_of_current_month);

    let mut dates = Vec::new();
    let mut date = start_date;
    while date <= today {
        dates.push(date);
        date = date + Days::new(1);
    }
    dates
}

/// One value per day in `dates`, ascending. A day with no entries shows 0
/// calories; target calories and weight forward-fill from the last known day,
/// since the user simply didn't touch those on a day without a `Day` row.
fn trailing_trend(
    dates: &[NaiveDate],
    days: &[DaySchema],
    entries: &[EntrySchema],
) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let mut sorted_days: Vec<&DaySchema> = days.iter().collect();
    sorted_days.sort_by_key(|day| day.date);
    let mut day_cursor = sorted_days.into_iter().peekable();

    let mut calories_series = Vec::with_capacity(dates.len());
    let mut target_series = Vec::with_capacity(dates.len());
    let mut weight_series = Vec::with_capacity(dates.len());

    let mut last_target: Option<i32> = None;
    let mut last_weight: Option<f32> = None;

    for &date in dates {
        while day_cursor.peek().is_some_and(|day| day.date <= date) {
            let day = day_cursor.next().expect("peeked Some above");
            last_target = Some(day.target_calories);
            last_weight = day.weight_kg.or(last_weight);
        }

        target_series.push(last_target.unwrap_or(0) as f64);
        weight_series.push(last_weight.unwrap_or(0.0) as f64);

        let calories: i32 = entries
            .iter()
            .filter(|entry| entry.date == date)
            .map(|entry| entry.calories)
            .sum();
        calories_series.push(calories as f64);
    }

    (calories_series, target_series, weight_series)
}

fn target_calories_as_of(date: NaiveDate, days: &[DaySchema]) -> Option<i32> {
    days.iter()
        .find(|day| day.date <= date)
        .map(|day| day.target_calories)
}

/// Calories eaten and calorie target for `date`, from one user's `days`/`entries`.
fn day_progress(date: NaiveDate, days: &[DaySchema], entries: &[EntrySchema]) -> (i32, i32) {
    let target_calories = target_calories_as_of(date, days).unwrap_or(0);

    let calories = entries
        .iter()
        .filter(|entry| entry.date == date)
        .map(|entry| entry.calories)
        .sum();

    (calories, target_calories)
}

/// Weight change between the first and the latest recorded day, in kg. `days`
/// is expected sorted newest first, matching what the days endpoints return.
fn weight_delta(days: &[DaySchema]) -> f32 {
    let latest_kg = days.iter().find_map(|day| day.weight_kg);
    let first_kg = days.iter().rev().find_map(|day| day.weight_kg);

    match (latest_kg, first_kg) {
        (Some(latest_kg), Some(first_kg)) => latest_kg - first_kg,
        _ => 0.0,
    }
}

#[component]
pub fn Home() -> Element {
    let session = use_auth();
    let user = session.user.read();
    let toast_api = use_toast();
    let navigator = use_navigator();

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

    let now = Local::now();
    let month = Month::from_zero_based(now.month0());
    let year = now.year();
    let today = now.date_naive();

    let mut is_target_calories_dialog_open = use_signal(|| false);
    let mut target_calories_input = use_signal(String::new);
    let mut is_saving_target_calories = use_signal(|| false);

    let mut home_data_resource = use_resource(move || {
        let current_user_id = session.user.read().as_ref().map(|user| user.id);

        async move {
            let Some(current_user_id) = current_user_id else {
                return HomeData {
                    community: vec![],
                    days: vec![],
                    entries: vec![],
                };
            };

            let days = match day::list().await {
                Ok(days) => days,
                Err(error) => {
                    toast_api.error(
                        "Errore".to_string(),
                        ToastOptions::new()
                            .description(error_message(&error))
                            .duration(Duration::from_secs(20)),
                    );
                    vec![]
                }
            };

            let entries = match entry::list().await {
                Ok(entries) => entries,
                Err(error) => {
                    toast_api.error(
                        "Errore".to_string(),
                        ToastOptions::new()
                            .description(error_message(&error))
                            .duration(Duration::from_secs(20)),
                    );
                    vec![]
                }
            };

            let users = match users::list().await {
                Ok(users) => users,
                Err(error) => {
                    toast_api.error(
                        "Errore".to_string(),
                        ToastOptions::new()
                            .description(error_message(&error))
                            .duration(Duration::from_secs(20)),
                    );
                    vec![]
                }
            };

            let mut community = Vec::with_capacity(users.len());

            for member in users {
                // The signed-in user's own days/entries are already fetched
                // above; everyone else's are public read data, fetched
                // on-demand through the community-scoped endpoints.
                let (calories, target_calories, member_weight_delta) =
                    if member.id == current_user_id {
                        let (calories, target_calories) = day_progress(today, &days, &entries);
                        (calories, target_calories, weight_delta(&days))
                    } else {
                        let member_days = match day::list_for_user(member.name.clone()).await {
                            Ok(days) => days,
                            Err(error) => {
                                toast_api.error(
                                    "Errore".to_string(),
                                    ToastOptions::new()
                                        .description(error_message(&error))
                                        .duration(Duration::from_secs(20)),
                                );
                                vec![]
                            }
                        };

                        let member_entries = match entry::list_for_user(member.name.clone()).await {
                            Ok(entries) => entries,
                            Err(error) => {
                                toast_api.error(
                                    "Errore".to_string(),
                                    ToastOptions::new()
                                        .description(error_message(&error))
                                        .duration(Duration::from_secs(20)),
                                );
                                vec![]
                            }
                        };

                        let (calories, target_calories) =
                            day_progress(today, &member_days, &member_entries);
                        (calories, target_calories, weight_delta(&member_days))
                    };

                community.push(CommunityMember {
                    user: member,
                    weight_delta: member_weight_delta,
                    calories,
                    target_calories,
                });
            }

            HomeData {
                community,
                days,
                entries,
            }
        }
    });

    if *session.is_loading.read() {
        return rsx! { div { class: "p-8 pt-20 flex flex-col gap-7 max-w-5xl mx-auto", } };
    }

    let Some(user) = user.as_ref() else {
        toast_api.error(
            "Error".to_string(),
            ToastOptions::new()
                .description("You need to be logged in")
                .duration(Duration::from_secs(20)),
        );
        navigator.push("/login");
        return rsx! {};
    };

    let home_data_state = home_data_resource.read();
    let Some(home_data) = home_data_state.as_ref() else {
        return rsx! { div { class: "p-8 pt-20 flex flex-col gap-7 max-w-5xl mx-auto", } };
    };

    let today_has_day = home_data.days.iter().any(|day| day.date == today);
    let today_target_calories = target_calories_as_of(today, &home_data.days);
    let today_target_calories_label = today_target_calories
        .map(|value| value.to_string())
        .unwrap_or_else(|| "–".to_string());
    let today_calories: i32 = home_data
        .entries
        .iter()
        .filter(|entry| entry.date == today)
        .map(|entry| entry.calories)
        .sum();

    let latest_weight = home_data
        .days
        .iter()
        .find_map(|day| day.weight_kg.map(|kg| (kg, day.date)));
    let first_weight = home_data
        .days
        .iter()
        .rev()
        .find_map(|day| day.weight_kg.map(|kg| (kg, day.date)));
    let current_kg = latest_weight.map(|(kg, _)| kg);
    let starting_kg = first_weight.map(|(kg, _)| kg);
    let starting_month =
        first_weight.map(|(_, date)| Month::from_zero_based(date.month0()).full_name());
    let target_kg = user.target_weight_kg;

    let weight_progress = match (starting_kg, current_kg, target_kg) {
        (Some(starting_kg), Some(current_kg), Some(target_kg)) => {
            let percent: f64 = 100.0
                - (100.0 / ((starting_kg - target_kg).abs() / (current_kg - target_kg).abs()))
                    as f64;
            Some((starting_kg, current_kg, target_kg, percent))
        }
        _ => None,
    };

    let chart_dates = trailing_month_dates(today);
    let (calories_series, target_calories_series, weight_series) =
        trailing_trend(&chart_dates, &home_data.days, &home_data.entries);

    let user_name = user.name.clone();
    let handle_save_target_calories = move || {
        let user_name = user_name.clone();
        async move {
            let Ok(target_calories) = target_calories_input().trim().parse::<i32>() else {
                toast_api.error(
                    "Errore".to_string(),
                    ToastOptions::new()
                        .description("Inserisci un numero di calorie valido")
                        .duration(Duration::from_secs(20)),
                );
                return;
            };

            is_saving_target_calories.set(true);

            let result = if today_has_day {
                day::update_target_calories(UpdateDayTargetCaloriesSchema {
                    user_name,
                    date: today,
                    target_calories,
                })
                .await
                .map(|_| ())
            } else {
                day::create(CreateDaySchema {
                    date: today,
                    user_name,
                    weight_kg: None,
                    target_calories: Some(target_calories),
                    notes: None,
                })
                .await
                .map(|_| ())
            };

            is_saving_target_calories.set(false);

            match result {
                Ok(()) => {
                    is_target_calories_dialog_open.set(false);
                    home_data_resource.restart();
                    toast_api.success(
                        "Fatto".to_string(),
                        ToastOptions::new()
                            .description("Obiettivo calorico aggiornato")
                            .duration(Duration::from_secs(20)),
                    );
                }
                Err(error) => {
                    toast_api.error(
                        "Errore".to_string(),
                        ToastOptions::new()
                            .description(error_message(&error))
                            .duration(Duration::from_secs(20)),
                    );
                }
            }
        }
    };

    rsx! {
        div {
            class: "p-8 pt-20 flex flex-col gap-7 max-w-5xl mx-auto",
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
                                    if let (Some(current_kg), Some(starting_kg)) = (current_kg, starting_kg) {
                                        p {
                                            class: "font-heading text-2xl mb-1",
                                            "{current_kg:.1} kg"
                                        }
                                        p {
                                            class: "text-xs text-primary-light",
                                            "{(current_kg - starting_kg):.1} kg da {starting_month.unwrap_or_default()}"
                                        }
                                    } else {
                                        p {
                                            class: "font-heading text-2xl mb-1",
                                            "—"
                                        }
                                        p {
                                            class: "text-xs text-primary-light",
                                            "Nessun peso registrato"
                                        }
                                    }
                                }
                                div {
                                    class: "text-right",
                                    p {
                                        class: "font-heading text-2xl mb-1",
                                        "{user.streak}"
                                    }
                                    p {
                                        class: "text-xs text-primary-light",
                                        "giorni di fila"
                                    }
                                }
                                div {
                                    class: "col-span-2",
                                    Separator {
                                        horizontal: true
                                    }
                                }
                                div {
                                    p {
                                        class: "font-heading text-2xl mb-1",
                                        "{today_calories} / {today_target_calories_label}"
                                    }
                                    p {
                                        class: "text-xs text-primary-light",
                                        "kcal oggi / obiettivo"
                                    }
                                }
                                div {
                                    class: "flex items-center justify-end",
                                    Button {
                                        type: "button",
                                        variant: ButtonVariant::Outline,
                                        size: ButtonSize::Sm,
                                        onclick: move |_| {
                                            target_calories_input.set(
                                                today_target_calories
                                                    .map(|value| value.to_string())
                                                    .unwrap_or_default(),
                                            );
                                            is_target_calories_dialog_open.set(true);
                                        },
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
                            if let (Some(current_kg), Some(starting_kg)) = (current_kg, starting_kg) {
                                p {
                                    class: "font-heading text-2xl mb-1",
                                    "{current_kg:.1} kg"
                                }
                                p {
                                    class: "text-xs text-primary-light",
                                    "{(current_kg - starting_kg):.1} kg da {starting_month.unwrap_or_default()}"
                                }
                            } else {
                                p {
                                    class: "font-heading text-2xl mb-1",
                                    "—"
                                }
                                p {
                                    class: "text-xs text-primary-light",
                                    "Nessun peso registrato"
                                }
                            }
                        }
                        div {
                            Separator {
                                horizontal: false
                            }
                        }
                        div {
                            class: "cursor-pointer",
                            role: "button",
                            onclick: move |_| {
                                target_calories_input.set(
                                    today_target_calories
                                        .map(|value| value.to_string())
                                        .unwrap_or_default(),
                                );
                                is_target_calories_dialog_open.set(true);
                            },
                            p {
                                class: "font-heading text-2xl mb-1",
                                "{today_calories} / {today_target_calories_label}"
                            }
                            p {
                                class: "text-xs text-primary-light",
                                "kcal oggi / obiettivo"
                            }
                        }
                        div {
                            Separator {
                                horizontal: false
                            }
                        }
                        div {
                            p {
                                class: "font-heading text-2xl mb-1",
                                "{user.streak}"
                            }
                            p {
                                class: "text-xs text-primary-light",
                                "giorni di fila"
                            }
                        }
                    }
                    Dialog {
                        open: is_target_calories_dialog_open(),
                        on_open_change: move |v| is_target_calories_dialog_open.set(v),
                        DialogTitle {
                            "Aggiorna l'obiettivo calorico"
                        }
                        DialogDescription {
                            form {
                                onsubmit: move |e| {
                                    e.prevent_default();
                                    handle_save_target_calories()
                                },
                                div {
                                    class: "space-y-2 mb-6",
                                    Label {
                                        html_for: "target_calories",
                                        "Nuovo obiettivo (kcal/giorno)"
                                    }
                                    Input {
                                        id: "target_calories",
                                        name: "target_calories",
                                        value: "{target_calories_input}",
                                        oninput: move |e: FormEvent| target_calories_input.set(e.value()),
                                    }
                                }
                                div {
                                    class: "flex justify-end items-center gap-4",
                                    Button {
                                        type: "button",
                                        onclick: move |_| is_target_calories_dialog_open.set(false),
                                        variant: ButtonVariant::Outline,
                                        disabled: is_saving_target_calories(),
                                        "Annulla"
                                    }
                                    Button {
                                        type: "submit",
                                        variant: ButtonVariant::Primary,
                                        disabled: is_saving_target_calories(),
                                        "Salva"
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Separator {
                class: "opacity-20"
            }
            Card {
                Accordion {
                    class: "w-full",
                    AccordionItem {
                        default_open: true,
                        index: 0,
                        AccordionTrigger {
                            div {
                                class: "pb-4",
                                p {
                                    class: "text-accent text-xs font-semibold",
                                    "COMMUNITY"
                                }
                                p {
                                    class: "font-heading text-xl mb-3",
                                    "Andamento del gruppo"
                                }
                                p {
                                    class: "text-sm text-primary-light",
                                    "Tocca una persona per vedere il suo diario e i suoi progressi."
                                }
                            }
                        }
                        AccordionContent {
                            for (index, member) in home_data.community.iter().enumerate() {
                                UserRow {
                                    key: "{member.user.id}",
                                    index: index as i32 + 1,
                                    name: member.user.name.clone(),
                                    streak: member.user.streak,
                                    month: month.full_name().to_string(),
                                    weight_delta: member.weight_delta,
                                    calories: member.calories,
                                    target_calories: member.target_calories,
                                    selected: member.user.id == user.id,
                                }
                            }
                        }
                    }
                }
            }
            if let Some((starting_kg, current_kg, target_kg, percent)) = weight_progress {
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
                        "Mancano {(current_kg - target_kg).abs():.1} kg all'obiettivo"
                    }
                }
            }
            Card {
                p {
                    class: "text-accent text-xs font-semibold",
                    // TODO: Seguirà l'utente selezionato, una volta che esisterà.
                    "IL TUO ANDAMENTO"
                }
                p {
                    class: "font-heading text-xl",
                    "Ultimi 30 giorni"
                }
                p {
                    class: "text-sm text-primary-light mb-6",
                    "Passa il cursore sul grafico per confrontare calorie e peso di un singolo giorno."
                }
                MonthlyChart {
                    series: vec![
                        ChartSeries::new("Kcal assunte", " kcal", calories_series).with_floor(0.0),
                        ChartSeries::new("Obiettivo kcal", " kcal", target_calories_series)
                            .with_color("#6B665E")
                            .dashed(),
                        ChartSeries::new("Peso", " kg", weight_series)
                            .with_decimals(1)
                            .with_floor(0.0),
                    ],
                }
            }
            div {
                class: "flex justify-between items-center mt-10",
                h2 {
                    class: "font-heading text-3xl",
                    "Diario alimentare"
                }
                Button {
                    class: "font-heading text-xl",
                    Plus {
                        size: "2em"
                    }
                    "Nuova voce"
                }
            }
            Card {
                if home_data.days.is_empty() {
                    p {
                        class: "text-sm text-primary-light",
                        "Nessun giorno registrato."
                    }
                } else {
                    div {
                        class: "space-y-4",
                        for (index, day) in home_data.days.iter().enumerate() {
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
                                    ingested_calories: home_data
                                        .entries
                                        .iter()
                                        .filter(|entry| entry.date == day.date)
                                        .map(|entry| entry.calories)
                                        .sum::<i32>(),
                                    target_calories: day.target_calories,
                                    notes: day.notes.clone(),
                                    for entry in home_data.entries.iter().filter(|entry| entry.date == day.date) {
                                        EntryRow {
                                            key: "{entry.id}",
                                            name: entry.name.clone(),
                                            calories: entry.calories,
                                            notes: entry.notes.clone().unwrap_or_default(),
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
