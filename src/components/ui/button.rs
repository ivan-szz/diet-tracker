use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

const BUTTON_CLASSES: &str = "inline-flex shrink-0 cursor-pointer items-center justify-center gap-2 whitespace-nowrap rounded-full border-0 p-0 font-[inherit] text-sm leading-5 font-medium transition-colors duration-100 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 hover:not-disabled:cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent/40 disabled:cursor-not-allowed disabled:opacity-50 aria-invalid:ring-2 aria-invalid:ring-red-700/40";

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
            ButtonVariant::Primary => "bg-accent text-background hover:not-disabled:bg-accent/90",
            ButtonVariant::Secondary => "bg-secondary text-background hover:not-disabled:bg-secondary-light",
            ButtonVariant::Destructive => "bg-red-700 text-white hover:not-disabled:bg-red-800 focus-visible:ring-red-700/40",
            ButtonVariant::Outline => "bg-transparent text-primary border border-1 border-primary rounded-full hover:not-disabled:bg-background",
            ButtonVariant::Ghost => "bg-transparent text-primary hover:not-disabled:bg-accent/10",
            ButtonVariant::Link => "bg-transparent text-accent underline-offset-4 hover:not-disabled:underline",
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
            ButtonSize::Xs => "h-6 gap-1 px-2 text-xs leading-4 has-[>svg]:px-1.5 [&_svg:not([class*='size-'])]:size-3",
            ButtonSize::Sm => "h-8 gap-1.5 px-3 has-[>svg]:px-2.5",
            ButtonSize::Default => "h-9 px-4 py-2 has-[>svg]:px-3",
            ButtonSize::Lg => "h-10 px-6 has-[>svg]:px-4",
            ButtonSize::Icon => "size-9",
            ButtonSize::IconXs => "size-6 [&_svg:not([class*='size-'])]:size-3",
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
