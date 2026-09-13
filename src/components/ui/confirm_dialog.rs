use crate::components::ui::{
    button::{Button, ButtonVariant},
    dialog::{Dialog, DialogDescription, DialogTitle},
};
use dioxus::prelude::*;

/// Asks for confirmation before a destructive action (delete, revoke, ...).
///
/// The caller owns the async lifecycle: `on_confirm` fires when the action
/// button is pressed, but the dialog does not close itself. Set `loading`
/// while the action is in flight and close it (via `on_open_change`) only
/// once it actually succeeds, the same way the target-calories dialog does.
#[derive(Clone, PartialEq, Props)]
pub struct ConfirmDialogProps {
    pub open: bool,
    pub on_open_change: EventHandler<bool>,

    pub title: String,
    pub description: String,

    #[props(default = "Conferma".to_string())]
    pub confirm_label: String,
    #[props(default = "Annulla".to_string())]
    pub cancel_label: String,

    pub on_confirm: EventHandler<()>,

    #[props(default = false)]
    pub loading: bool,
}

#[component]
pub fn ConfirmDialog(props: ConfirmDialogProps) -> Element {
    rsx! {
        Dialog {
            open: props.open,
            on_open_change: props.on_open_change,
            DialogTitle {
                "{props.title}"
            }
            DialogDescription {
                "{props.description}"
            }
            div {
                class: "flex justify-end items-center gap-4",
                Button {
                    type: "button",
                    onclick: move |_| props.on_open_change.call(false),
                    variant: ButtonVariant::Outline,
                    disabled: props.loading,
                    "{props.cancel_label}"
                }
                Button {
                    type: "button",
                    onclick: move |_| props.on_confirm.call(()),
                    variant: ButtonVariant::Destructive,
                    disabled: props.loading,
                    "{props.confirm_label}"
                }
            }
        }
    }
}
