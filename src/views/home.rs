use chrono::{Datelike, Local};
use dioxus::prelude::*;
use dioxus_icons::lucide::{ArrowRight};

use crate::{components::{UserRow, ui::{accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger}, card::Card, progress::Progress, separator::Separator}}, schema::user::UserSchema};

const MONTHS: [&str; 12] = [
    "GENNAIO",
    "FEBBRAIO",
    "MARZO",
    "APRILE",
    "MAGGIO",
    "GIUGNO",
    "LUGLIO",
    "AGOSTO",
    "SETTEMBRE",
    "OTTOMBRE",
    "NOVEMBRE",
    "DICEMBRE",
];

#[component]
pub fn Home() -> Element {
    let now = Local::now();
    let month = MONTHS[now.month0() as usize];
    let year = now.year();

    let target_kg: f32 = 65.0;
    let current_kg: f32 = 92.7;
    let starting_kg: f32 = 95.5;
    let percent = 100.0 - (100.0/((starting_kg - target_kg).abs()/(current_kg - target_kg).abs())) as f64;

    rsx! {
        div {
            class: "p-8 pt-20 flex flex-col gap-7",
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
                            p {
                                class: "font-heading text-2xl mb-1",
                                "700 / 1800"
                            }
                            // TODO: Add modal
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
                    class: "text-accent text-xs font-semibold mt-4",
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
        }
    }
}
