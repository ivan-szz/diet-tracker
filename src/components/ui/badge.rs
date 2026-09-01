use dioxus::prelude::*;
use dioxus_icons::lucide::BadgeCheck;

#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum BadgeVariant {
    #[default]
    Primary,
    Secondary,
    Destructive,
    Outline,
}

impl BadgeVariant {
    pub fn class(&self) -> &'static str {
        match self {
            BadgeVariant::Primary => "primary",
            BadgeVariant::Secondary => "secondary",
            BadgeVariant::Destructive => "destructive",
            BadgeVariant::Outline => "outline",
        }
    }

    fn classes(&self) -> &'static str {
        match self {
            BadgeVariant::Primary => {
                "bg-secondary-lighter text-secondary"
            }
            BadgeVariant::Secondary => {
                "bg-[var(--primary-color-5)] text-[var(--secondary-color-1)]"
            }
            BadgeVariant::Destructive => {
                "bg-[var(--primary-error-color)] text-[var(--contrast-error-color)]"
            }
            BadgeVariant::Outline => "border border-[var(--primary-color-6)] bg-[var(--primary-color)] text-[var(--secondary-color-4)]",
        }
    }
}

/// The props for the [`Badge`] component.
#[derive(Props, Clone, PartialEq)]
pub struct BadgeProps {
    #[props(default)]
    pub variant: BadgeVariant,

    /// Additional attributes to extend the badge element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the badge element
    pub children: Element,
}

#[component]
pub fn Badge(props: BadgeProps) -> Element {
    rsx! {
        BadgeElement {
            "padding": true,
            variant: props.variant,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
fn BadgeElement(props: BadgeProps) -> Element {
    rsx! {
        span {
            class: format!("inline-flex min-w-5 items-center justify-center gap-1 rounded-full px-2 text-sm {}", props.variant.classes()),
            "data-style": props.variant.class(),
            ..props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn VerifiedIcon() -> Element {
    rsx! {
        BadgeCheck {
            size: "12px",
            stroke: "var(--secondary-color-4)",
        }
    }
}
