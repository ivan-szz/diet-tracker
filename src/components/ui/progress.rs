use dioxus::prelude::*;
use dioxus_primitives::progress::{self, ProgressProps};

#[component]
pub fn Progress(props: ProgressProps) -> Element {
    rsx! {
        progress::Progress {
            class: "group relative h-2 min-w-50 overflow-hidden rounded-full bg-[var(--primary-color-5)]",
            value: props.value,
            max: props.max,
            attributes: props.attributes,
            progress::ProgressIndicator {
                class: "h-full w-[var(--progress-value,0%)] bg-[var(--secondary-color-1)] transition-[width] duration-250 ease-in-out group-data-[state=indeterminate]:w-1/2 group-data-[state=indeterminate]:animate-progress-indeterminate"
            }
        }
    }
}
