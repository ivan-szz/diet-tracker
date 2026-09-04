use crate::components::ui::button::ButtonVariant;
use crate::components::ui::dialog::{Dialog, DialogDescription, DialogTitle};
use crate::components::ui::input::Input;
use crate::components::ui::label::Label;
use crate::{
    components::{
        ui::{
            accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger},
            button::Button,
            card::Card,
            chart::{Chart, ChartSeries},
            progress::Progress,
            separator::Separator,
        },
        DayBlock, EntryRow, UserRow,
    },
    utils::constants::{MONTHS, SHORT_MONTHS},
};
use chrono::{Datelike, Days, Local};
use dioxus::prelude::*;
use dioxus_icons::lucide::{ArrowRight, Plus};

const HISTORY_DAYS: u64 = 30;

// TODO: Questi tre andamenti sono segnaposto, arriveranno dal repo dei giorni dell'utente selezionato.
const CALORIES: [f64; 30] = [
    1450.0, 1720.0, 1980.0, 1610.0, 1290.0, 2050.0, 1870.0, 1540.0, 1660.0, 1930.0, 1380.0, 1750.0,
    1610.0, 2110.0, 1480.0, 1690.0, 1820.0, 1350.0, 1570.0, 4000.0, 1710.0, 1440.0, 1880.0, 1620.0,
    1300.0, 1990.0, 1750.0, 1530.0, 1680.0, 1420.0,
];
const TARGET_CALORIES: [f64; 30] = [
    2000.0, 2000.0, 2000.0, 2000.0, 2000.0, 2000.0, 2000.0, 2000.0, 2000.0, 2000.0, 2000.0, 2000.0,
    1800.0, 1800.0, 1800.0, 1800.0, 1800.0, 1800.0, 1800.0, 1800.0, 1800.0, 1800.0, 1800.0, 1800.0,
    1800.0, 1800.0, 1800.0, 1800.0, 1800.0, 1800.0,
];
const WEIGHT: [f64; 30] = [
    95.5, 95.4, 95.6, 95.2, 95.0, 95.1, 94.8, 94.6, 94.7, 94.4, 94.2, 94.3, 94.0, 93.8, 93.9, 93.7,
    93.5, 93.6, 93.4, 93.3, 93.5, 93.2, 93.0, 93.1, 92.9, 92.8, 93.0, 92.8, 92.6, 92.7,
];

#[component]
pub fn Home() -> Element {
    let now = Local::now();
    let month = MONTHS[now.month0() as usize];
    let year = now.year();

    // La riga in basso del grafico: gli ultimi giorni, dal più vecchio a oggi.
    let days: Vec<String> = (0..HISTORY_DAYS)
        .rev()
        .map(|back| {
            let day = now.date_naive() - Days::new(back);
            format!("{} {}", day.day(), SHORT_MONTHS[day.month0() as usize])
        })
        .collect();

    let target_kg: f32 = 65.0;
    let current_kg: f32 = 92.7;
    let starting_kg: f32 = 95.5;
    let percent =
        100.0 - (100.0 / ((starting_kg - target_kg).abs() / (current_kg - target_kg).abs())) as f64;

    let mut is_target_calories_dialog_open = use_signal(|| false);

    rsx! {
        div {
            class: "p-8 pt-20 flex flex-col gap-7 max-w-5xl mx-auto",
            div {
                p {
                    class: "text-accent text-xs font-semibold mb-2",
                    "DIARIO ALIMENTARE · {month} {year}"
                }
                div {
                    class: "flex justify-between items-end",
                    div {
                        h1 {
                            class: "font-heading text-5xl mb-3",
                            // TODO: Questo diventerà uno state che mostrerà l'utente attualmente selezionato
                            "Tu"
                        }
                        p {
                            class: "text-primary-light",
                            "Ultimi 30 giorni di monitoraggio"
                        }
                    }
                    div {
                        class: "flex gap-8 content-center",
                        div {
                            p {
                                class: "font-heading text-2xl mb-1",
                                "78.5 kg"
                            }
                            p {
                                class: "text-xs text-primary-light",
                                "-4.9 kg da maggio"
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
                            onclick: move |_| is_target_calories_dialog_open.set(true),
                            p {
                                class: "font-heading text-2xl mb-1",
                                "700 / 1800"
                            }
                            p {
                                class: "text-xs text-primary-light",
                                "kcal oggi / obiettivo"
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
                                    onsubmit: move |e| e.prevent_default(),
                                    div {
                                        class: "space-y-2 mb-6",
                                        Label {
                                            html_for: "target_calories",
                                            "Nuovo obiettivo (kcal/giorno)"
                                        }
                                        Input {
                                            id: "target_calories",
                                            name: "target_calories",
                                            value: 1800
                                        }
                                    }
                                    div {
                                        class: "flex justify-end items-center gap-4",
                                        Button {
                                            type: "button",
                                            onclick: move |_| is_target_calories_dialog_open.set(false),
                                            variant: ButtonVariant::Outline,
                                            "Annulla"
                                        }
                                        Button {
                                            type: "submit",
                                            variant: ButtonVariant::Primary,
                                            "Salva"
                                        }
                                    }
                                }
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
                                "12"
                            }
                            p {
                                class: "text-xs text-primary-light",
                                "giorni di fila"
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
                            UserRow {
                                index: 1,
                                name: "Ivan Sozza",
                                streak: 13,
                                month: "agosto",
                                weight_delta: -2.6,
                                calories: 700,
                                target_calories: 1800,
                            }
                            UserRow {
                                index: 2,
                                name: "Claudio Bisio",
                                streak: 12,
                                month: "maggio",
                                weight_delta: -1.4,
                                calories: 400,
                                target_calories: 1500,
                                selected: true
                            }
                        }
                    }
                }
            }
            Card {
                p {
                    class: "text-accent text-xs font-semibold",
                    "OBIETTIVO PESO"
                }
                p {
                    class: "font-heading text-xl mb-3",
                    span {
                        class: "flex items-center gap-1",
                        "{starting_kg} kg"
                        ArrowRight {}
                        "{target_kg} kg"
                    }
                }
                div {
                    class: "relative w-full flex items-center",
                    p {
                        class: "absolute text-background font-heading z-10 -translate-x-full pr-3",
                        left: "{percent:.1}%",
                        "{percent:.1} %"
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
                Chart {
                    days: days,
                    series: vec![
                        ChartSeries::new("Calorie assunte", " kcal", CALORIES.to_vec()),
                        ChartSeries::new("Obiettivo calorie", " kcal", TARGET_CALORIES.to_vec())
                            .with_color("#6B665E")
                            .dashed(),
                        // Il peso resta a un decimale anche in una settimana di valori tondi.
                        ChartSeries::new("Peso", " kg", WEIGHT.to_vec()).with_decimals(1),
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
                // TODO: Iterare su una mappa giorni -> entry
                div {
                    class: "space-y-4",
                    DayBlock {
                        date: now.date_naive(),
                        ingested_calories: 1463,
                        target_calories: 1800,
                        EntryRow {
                            name: "Saikebon Manzo (Yakisoba)",
                            calories: 413,
                        }
                        EntryRow {
                            name: "Pizza",
                            calories: 1050,
                            notes: "Pranzo ufficio"
                        }
                    }
                    Separator {
                        // TODO: Renderizzare prima di tutti i day block tranne il primo
                        class: "opacity-20"
                    }
                    DayBlock {
                        date: now.date_naive() - Days::new(1),
                        ingested_calories: 1490,
                        target_calories: 1800,
                        EntryRow {
                            name: "Qualcos'altro",
                            calories: 840,
                        }
                        EntryRow {
                            name: "Un'altra cosa ancora",
                            calories: 650,
                        }
                    }
                }
            }
        }
    }
}
