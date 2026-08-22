use dioxus::prelude::*;
use dioxus_icons::lucide::Check;
use dioxus_primitives::checkbox::{self, CheckboxProps};

#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    rsx! {
        checkbox::Checkbox {
            class: "m-0 size-4 cursor-pointer rounded-sm border-0 bg-[var(--primary-color-3)] p-0 text-[var(--secondary-color-4)] shadow-[inset_0_0_0_1px_var(--primary-color-7)] data-[state=checked]:bg-[var(--secondary-color-2)] data-[state=checked]:text-[var(--primary-color)] data-[state=checked]:shadow-none focus-visible:shadow-[0_0_0_2px_var(--focused-border-color)]",
            checked: props.checked,
            default_checked: props.default_checked,
            required: props.required,
            disabled: props.disabled,
            name: props.name,
            value: props.value,
            on_checked_change: props.on_checked_change,
            attributes: props.attributes,
            checkbox::CheckboxIndicator { class: "flex items-center justify-center",
                Check { size: "1rem" }
            }
        }
    }
}
