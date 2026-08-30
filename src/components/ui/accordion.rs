use dioxus::prelude::*;
use dioxus_icons::lucide::ChevronDown;
use dioxus_primitives::accordion::{
    self, AccordionContentProps, AccordionItemProps, AccordionProps, AccordionTriggerProps,
};

#[component]
pub fn Accordion(props: AccordionProps) -> Element {
    rsx! {
        accordion::Accordion {
            class: "w-60 [contain:inline-size]",
            id: props.id,
            allow_multiple_open: props.allow_multiple_open,
            disabled: props.disabled,
            collapsible: props.collapsible,
            horizontal: props.horizontal,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn AccordionItem(props: AccordionItemProps) -> Element {
    rsx! {
        accordion::AccordionItem {
            class: "group mt-px box-border overflow-hidden border-b border-[var(--primary-color-6)] first:mt-0 last:border-b-0",
            disabled: props.disabled,
            default_open: props.default_open,
            on_change: props.on_change,
            on_trigger_click: props.on_trigger_click,
            index: props.index,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn AccordionTrigger(props: AccordionTriggerProps) -> Element {
    rsx! {
        accordion::AccordionTrigger {
            class: "flex w-full flex-row items-center justify-between border-0 bg-transparent py-4 text-left text-[var(--secondary-color-4)] outline-none hover:not-disabled:cursor-pointer focus-visible:shadow-[inset_0_0_0_2px_var(--focused-border-color)] disabled:cursor-not-allowed disabled:text-[var(--secondary-color-5)] disabled:opacity-50",
            id: props.id,
            attributes: props.attributes,
            {props.children}
            ChevronDown {
                class: "transition-transform duration-300 ease-in-out group-data-[open=true]:rotate-180",
                size: "20px",
                stroke: "var(--color-primary)",
            }
        }
    }
}

#[component]
pub fn AccordionContent(props: AccordionContentProps) -> Element {
    rsx! {
        accordion::AccordionContent {
            class: "h-0 overflow-hidden opacity-0 [interpolate-size:allow-keywords] transition-[height,opacity] duration-300 ease-[cubic-bezier(0.22,1,0.36,1)] data-[open=true]:h-auto data-[open=true]:animate-accordion-open data-[open=true]:opacity-100 motion-reduce:animate-none motion-reduce:transition-none",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}
