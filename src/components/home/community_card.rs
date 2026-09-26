use super::dates::month_name;
use crate::components::ui::accordion::{
    Accordion, AccordionContent, AccordionItem, AccordionTrigger,
};
use crate::components::ui::card::Card;
use crate::components::UserRow;
use crate::schema::stats::UserSummarySchema;
use dioxus::prelude::*;

#[component]
pub fn CommunityCard(members: Vec<UserSummarySchema>, current_user_id: i32) -> Element {
    rsx! {
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
                        for (index, member) in members.iter().enumerate() {
                            UserRow {
                                key: "{member.user.id}",
                                index: index as i32 + 1,
                                name: member.user.name.clone(),
                                streak: member.user.streak,
                                month: member.weight.starting_date.map(month_name).unwrap_or_default().to_string(),
                                weight_delta: member.weight.delta_kg,
                                calories: member.day.calories,
                                target_calories: member.day.target_calories.unwrap_or(0),
                                selected: member.user.id == current_user_id,
                            }
                        }
                    }
                }
            }
        }
    }
}
