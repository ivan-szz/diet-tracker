use dioxus::prelude::*;

#[component]
pub fn Segmented(
    #[props(default)] class: String,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "inline-flex overflow-hidden rounded-full border border-primary/15 {class}",
            role: "group",
            ..attributes,
            {children}
        }
    }
}

/// One radio option of a [`Segmented`] group. `onclick` fires on every
/// activation, including on an already-checked option, so it can reopen UI.
#[component]
pub fn SegmentedOption(
    name: String,
    checked: bool,
    onclick: EventHandler<MouseEvent>,
    children: Element,
) -> Element {
    rsx! {
        label {
            class: "inline-flex cursor-pointer items-center gap-1.5 px-3 py-[7px] text-[13px] not-first:border-l not-first:border-primary/15 has-checked:bg-accent has-checked:text-background not-has-checked:hover:bg-primary/5 has-focus-visible:outline-2 has-focus-visible:-outline-offset-2 has-focus-visible:outline-accent",
            input {
                class: "pointer-events-none absolute size-0 opacity-0",
                r#type: "radio",
                name,
                checked,
                onclick: move |e| onclick.call(e),
            }
            {children}
        }
    }
}
