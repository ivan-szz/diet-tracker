use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

const BUTTON_CLASSES: &str = "inline-flex shrink-0 cursor-default items-center justify-center gap-2 whitespace-nowrap rounded-lg border-0 p-0 font-[inherit] text-sm leading-5 font-medium outline-none transition-all duration-150 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 disabled:pointer-events-none disabled:opacity-50 focus-visible:border-[var(--focused-border-color)] focus-visible:shadow-[0_0_0_3px_color-mix(in_oklab,var(--focused-border-color)_50%,transparent)] aria-invalid:border-[var(--primary-error-color)] aria-invalid:focus-visible:shadow-[0_0_0_3px_color-mix(in_oklab,var(--primary-error-color)_20%,transparent)]";

#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Destructive,
    Outline,
    Ghost,
    Link,
}

impl ButtonVariant {
    pub fn class(&self) -> &'static str {
        match self {
            ButtonVariant::Primary => "primary",
            ButtonVariant::Secondary => "secondary",
            ButtonVariant::Destructive => "destructive",
            ButtonVariant::Outline => "outline",
            ButtonVariant::Ghost => "ghost",
            ButtonVariant::Link => "link",
        }
    }

    fn classes(&self) -> &'static str {
        match self {
            ButtonVariant::Primary => "bg-[var(--secondary-color-2)] text-[var(--primary-color)] hover:not-disabled:bg-[color-mix(in_oklab,var(--secondary-color-2)_90%,transparent)]",
            ButtonVariant::Secondary => "bg-[var(--primary-color-5)] text-[var(--secondary-color-1)] hover:not-disabled:bg-[color-mix(in_oklab,var(--primary-color-5)_80%,transparent)]",
            ButtonVariant::Destructive => "bg-[var(--primary-error-color)] text-[var(--contrast-error-color)] hover:not-disabled:bg-[color-mix(in_oklab,var(--primary-error-color)_90%,transparent)] focus-visible:shadow-[0_0_0_3px_color-mix(in_oklab,var(--primary-error-color)_20%,transparent)]",
            ButtonVariant::Outline => "border border-[var(--primary-color-6)] bg-[var(--primary-color)] text-[var(--secondary-color-4)] shadow-sm hover:not-disabled:bg-[var(--primary-color-5)] hover:not-disabled:text-[var(--secondary-color-1)]",
            ButtonVariant::Ghost => "bg-transparent text-[var(--secondary-color-4)] hover:not-disabled:bg-[var(--primary-color-5)] hover:not-disabled:text-[var(--secondary-color-1)]",
            ButtonVariant::Link => "bg-transparent text-[var(--secondary-color-2)] underline-offset-4 hover:not-disabled:underline",
        }
    }
}

#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum ButtonSize {
    Xs,
    Sm,
    #[default]
    Default,
    Lg,
    Icon,
    IconXs,
    IconSm,
    IconLg,
}

impl ButtonSize {
    pub fn class(&self) -> &'static str {
        match self {
            ButtonSize::Xs => "xs",
            ButtonSize::Sm => "sm",
            ButtonSize::Default => "default",
            ButtonSize::Lg => "lg",
            ButtonSize::Icon => "icon",
            ButtonSize::IconXs => "icon-xs",
            ButtonSize::IconSm => "icon-sm",
            ButtonSize::IconLg => "icon-lg",
        }
    }

    fn classes(&self) -> &'static str {
        match self {
            ButtonSize::Xs => "h-6 gap-1 rounded-lg px-2 text-xs leading-4 has-[>svg]:px-1.5 [&_svg:not([class*='size-'])]:size-3",
            ButtonSize::Sm => "h-8 gap-1.5 rounded-lg px-3 has-[>svg]:px-2.5",
            ButtonSize::Default => "h-9 px-4 py-2 has-[>svg]:px-3",
            ButtonSize::Lg => "h-10 rounded-lg px-6 has-[>svg]:px-4",
            ButtonSize::Icon => "size-9",
            ButtonSize::IconXs => "size-6 rounded-lg [&_svg:not([class*='size-'])]:size-3",
            ButtonSize::IconSm => "size-8",
            ButtonSize::IconLg => "size-10",
        }
    }
}

#[component]
pub fn Button(
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(extends=GlobalAttributes)]
    #[props(extends=button)]
    attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    onmousedown: Option<EventHandler<MouseEvent>>,
    onmouseup: Option<EventHandler<MouseEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    children: Element,
) -> Element {
    let base = attributes!(button {
        class: format!("{BUTTON_CLASSES} {} {}", variant.classes(), size.classes()),
        "data-style": variant.class(),
        "data-size": size.class(),
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        button {
            onclick: move |event| {
                if let Some(f) = &onclick {
                    f.call(event);
                }
            },
            onmousedown: move |event| {
                if let Some(f) = &onmousedown {
                    f.call(event);
                }
            },
            onmouseup: move |event| {
                if let Some(f) = &onmouseup {
                    f.call(event);
                }
            },
            onkeydown: move |event| {
                if let Some(f) = &onkeydown {
                    f.call(event);
                }
            },
            ..merged,
            {children}
        }
    }
}
