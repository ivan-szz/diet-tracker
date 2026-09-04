use dioxus::prelude::*;
use dioxus_primitives::dialog::{self, DialogDescriptionProps, DialogRootProps, DialogTitleProps};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[component]
pub fn Dialog(props: DialogRootProps) -> Element {
    let base = attributes!(div {
        class: "fixed top-1/2 left-1/2 z-1001 m-0 flex w-full max-w-[calc(100%-2rem)] -translate-x-1/2 -translate-y-1/2 flex-col gap-4 rounded-4xl bg-background-dark px-6 pt-8 pb-6 text-center font-sans text-primary shadow-lg sm:max-w-lg sm:text-left",
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        dialog::DialogRoot {
            class: "fixed inset-0 z-1000 bg-primary/30 opacity-0 will-change-[opacity] data-[state=closed]:animate-alert-out data-[state=open]:animate-alert-in",
            id: props.id,
            is_modal: props.is_modal,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            dialog::DialogContent {
                class: None,
                attributes: merged,
                {props.children}
            }
        }
    }
}

#[component]
pub fn DialogTitle(props: DialogTitleProps) -> Element {
    let base = attributes!(h2 {
        class: "m-0 font-heading text-xl text-primary",
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        dialog::DialogTitle {
            id: props.id,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn DialogDescription(props: DialogDescriptionProps) -> Element {
    let base = attributes!(p {
        class: "m-0 text-sm text-primary-light",
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        dialog::DialogDescription {
            id: props.id,
            attributes: merged,
            {props.children}
        }
    }
}
