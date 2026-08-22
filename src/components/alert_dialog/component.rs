use dioxus::prelude::*;
use dioxus_primitives::alert_dialog::{
    self, AlertDialogActionProps, AlertDialogActionsProps, AlertDialogCancelProps,
    AlertDialogDescriptionProps, AlertDialogRootProps, AlertDialogTitleProps,
};

#[component]
pub fn AlertDialog(props: AlertDialogRootProps) -> Element {
    rsx! {
        alert_dialog::AlertDialogRoot {
            class: "fixed inset-0 z-1000 bg-black/30 opacity-0 will-change-[opacity] data-[state=closed]:animate-alert-out data-[state=open]:animate-alert-in",
            id: props.id,
            default_open: props.default_open,
            open: props.open,
            on_open_change: props.on_open_change,
            attributes: props.attributes,
            alert_dialog::AlertDialogContent {
                class: "fixed top-1/2 left-1/2 z-1001 m-0 flex w-full max-w-[calc(100%-2rem)] -translate-x-1/2 -translate-y-1/2 flex-col gap-4 rounded-lg border border-[var(--primary-color-6)] bg-[var(--primary-color-2)] px-6 pt-8 pb-6 text-center font-sans text-[var(--secondary-color-4)] shadow-[0_2px_10px_rgb(0_0_0/18%)] sm:max-w-lg sm:text-left",
                {props.children}
            }
        }
    }
}

#[component]
pub fn AlertDialogTitle(props: AlertDialogTitleProps) -> Element {
    rsx! {
        alert_dialog::AlertDialogTitle {
            class: "m-0 text-xl font-bold text-[var(--secondary-color-4)]",
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn AlertDialogDescription(props: AlertDialogDescriptionProps) -> Element {
    rsx! {
        alert_dialog::AlertDialogDescription {
            class: "m-0 text-base text-[var(--secondary-color-5)]",
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn AlertDialogActions(props: AlertDialogActionsProps) -> Element {
    rsx! {
        alert_dialog::AlertDialogActions {
            class: "flex flex-col-reverse gap-3 sm:flex-row sm:justify-end",
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn AlertDialogCancel(props: AlertDialogCancelProps) -> Element {
    rsx! {
        alert_dialog::AlertDialogCancel {
            on_click: props.on_click,
            class: "cursor-pointer rounded-lg border border-[var(--primary-color-6)] bg-[var(--primary-color)] px-[18px] py-2 text-base text-[var(--secondary-color-4)] transition-colors duration-200 hover:bg-[var(--primary-color-4)] focus-visible:shadow-[0_0_0_2px_var(--focused-border-color)]",
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn AlertDialogAction(props: AlertDialogActionProps) -> Element {
    rsx! {
        alert_dialog::AlertDialogAction {
            class: "cursor-pointer rounded-lg border border-[var(--primary-error-color)] bg-[var(--primary-error-color)] px-[18px] py-2 text-base text-[var(--contrast-error-color)] transition-colors duration-200 hover:bg-[var(--secondary-error-color)] focus-visible:shadow-[0_0_0_2px_var(--focused-border-color)]",
            on_click: props.on_click,
            attributes: props.attributes,
            {props.children}
        }
    }
}
