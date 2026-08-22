use dioxus::prelude::*;
use dioxus_primitives::switch::{self, SwitchProps};

#[component]
pub fn Switch(props: SwitchProps) -> Element {
    rsx! {
        switch::Switch {
            class: "group relative h-[1.15rem] w-8 cursor-pointer appearance-none rounded-full border-0 bg-[var(--primary-color-6)] p-0 outline-none transition-[background-color,box-shadow] duration-150 data-[state=checked]:bg-[var(--secondary-color-2)] data-[disabled=true]:cursor-not-allowed data-[disabled=true]:opacity-50 focus-visible:shadow-[0_0_0_3px_color-mix(in_oklab,var(--focused-border-color)_50%,transparent)]",
            checked: props.checked,
            default_checked: props.default_checked,
            disabled: props.disabled,
            required: props.required,
            name: props.name,
            value: props.value,
            on_checked_change: props.on_checked_change,
            attributes: props.attributes,
            switch::SwitchThumb {
                class: "block size-[calc(1.15rem-2px)] translate-x-px rounded-full bg-[var(--primary-color)] transition-transform duration-150 will-change-transform group-data-[state=checked]:translate-x-[calc(2rem-1px-(1.15rem-2px))]"
            }
        }
    }
}
