use super::data::CommunityMember;
use crate::components::ui::accordion::{
    Accordion, AccordionContent, AccordionItem, AccordionTrigger,
};
use crate::components::ui::card::Card;
use crate::components::UserRow;
use dioxus::prelude::*;

#[component]
pub fn CommunityCard(members: Vec<CommunityMember>, current_user_id: i32) -> Element {
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
                                month: member.starting_month.clone().unwrap_or_default(),
                                weight_delta: member.weight_delta,
                                calories: member.calories,
                                target_calories: member.target_calories,
                                selected: member.user.id == current_user_id,
                            }
                        }
                    }
                }
            }
        }
    }
}
