use dioxus::prelude::*;
use dioxus_primitives::alert_dialog::{
    self, AlertDialogActionProps, AlertDialogActionsProps, AlertDialogCancelProps,
    AlertDialogDescriptionProps, AlertDialogRootProps, AlertDialogTitleProps,
};

#[component]
pub fn AlertDialog(props: AlertDialogRootProps) -> Element {
    rsx! {
        alert_dialog::AlertDialogRoot {
            class: "fixed inset-0 z-1000 bg-primary/30 opacity-0 will-change-[opacity] data-[state=closed]:animate-alert-out data-[state=open]:animate-alert-in",
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
            class: "m-0 font-heading text-xl text-primary",
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn AlertDialogDescription(props: AlertDialogDescriptionProps) -> Element {
    rsx! {
        alert_dialog::AlertDialogDescription {
            class: "m-0 text-sm text-primary-light",
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
            class: "cursor-pointer rounded-full border border-accent/20 bg-background px-[18px] py-2 font-heading text-base text-primary transition-colors duration-100 hover:bg-accent/10 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent/40",
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn AlertDialogAction(props: AlertDialogActionProps) -> Element {
    rsx! {
        alert_dialog::AlertDialogAction {
            class: "cursor-pointer rounded-full border-0 bg-accent px-[18px] py-2 font-heading text-base text-background transition-colors duration-100 hover:bg-accent/90 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent/40",
            on_click: props.on_click,
            attributes: props.attributes,
            {props.children}
        }
    }
}
