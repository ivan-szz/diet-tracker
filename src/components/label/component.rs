use dioxus::prelude::*;
use dioxus_primitives::label::{self, LabelProps};
#[component]
pub fn Label(props: LabelProps) -> Element {
    rsx! {
        label::Label {
            class: "flex items-center text-[0.8rem] leading-none text-[var(--secondary-color-4)]",
            html_for: props.html_for,
            attributes: props.attributes,
            {props.children}
        }
    }
}
