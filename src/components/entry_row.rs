use dioxus::prelude::*;
use dioxus_icons::lucide::X;

use crate::components::ui::{
    button::{Button, ButtonVariant},
    separator::Separator,
};

#[derive(Clone, PartialEq, Props)]
pub struct EntryRowProps {
    pub name: String,
    pub calories: i32,

    #[props(default)]
    pub notes: String,
}

#[component]
pub fn EntryRow(props: EntryRowProps) -> Element {
    rsx! {
        div {
            Separator {
                class: "opacity-20"
            }
            div {
                class: "flex items-center justify-between pt-2",
                p {
                    class: "font-semibold",
                    "{props.name}"
                }
                div {
                    class: "flex items-center gap-4",
                    p {
                        class: "font-semibold",
                        "{props.calories}"
                    }
                    Button {
                        variant: ButtonVariant::Ghost,
                        X {
                            size: "1.25em"
                        }
                    }
                }
            }
            if !props.notes.is_empty() {
                p {
                    class: "text-xs text-primary-light",
                    "{props.notes}"
                }
            }
        }
    }
}
