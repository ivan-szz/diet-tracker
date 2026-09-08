use dioxus::document::eval;
use dioxus::prelude::*;
use dioxus_html::{MouseData, SerializedMouseData};
use dioxus_primitives::toast::{
    self, Toast, ToastContentProps, ToastDescriptionProps, ToastProps, ToastTitleProps,
};
use std::rc::Rc;
use std::time::Duration;

const TOAST_BASE: &str = "group/toast relative flex min-h-16 w-80 items-start justify-between gap-3 overflow-hidden rounded-2xl px-5 py-4 font-sans shadow-lg ring-1 will-change-[transform,opacity] data-[type=success]:bg-secondary-lighter data-[type=success]:text-primary data-[type=success]:ring-secondary/25 data-[type=error]:bg-red-100 data-[type=error]:text-red-950 data-[type=error]:ring-red-500/30 data-[type=warning]:bg-accent/15 data-[type=warning]:text-primary data-[type=warning]:ring-accent/30 data-[type=info]:bg-background-dark data-[type=info]:text-primary data-[type=info]:ring-primary/10";

/// Kept a hair longer than the `toast-out` keyframe so the animation finishes on screen.
const EXIT_MS: u32 = 190;

#[component]
fn StyledToast(props: ToastProps) -> Element {
    let mut leaving = use_signal(|| false);
    let on_close = props.on_close;

    // Play the exit animation first, then let the provider drop the toast.
    let close = use_callback(move |_: ()| {
        if leaving() {
            return;
        }
        leaving.set(true);
        spawn(async move {
            let mut wait = eval(&format!(
                "await new Promise((r) => setTimeout(r, {EXIT_MS})); dioxus.send(true);"
            ));
            let _ = wait.recv::<bool>().await;
            on_close.call(Event::new(
                Rc::new(MouseData::new(SerializedMouseData::default())),
                false,
            ));
        });
    });

    // Drive auto-dismiss ourselves so timed-out toasts fade out like manually closed ones.
    use_hook(move || {
        if props.permanent {
            return;
        }
        let ms = props
            .duration
            .unwrap_or(Duration::from_secs(5))
            .as_millis()
            .min(u128::from(u32::MAX)) as u32;
        spawn(async move {
            let mut wait = eval(&format!(
                "await new Promise((r) => setTimeout(r, {ms})); dioxus.send(true);"
            ));
            let _ = wait.recv::<bool>().await;
            close.call(());
        });
    });

    // `animate-toast-in` bakes the per-index delay into its `animation` shorthand
    // (see tailwind.css), so grouped toasts cascade in instead of popping together.
    let motion = if leaving() {
        "animate-toast-out"
    } else {
        "animate-toast-in"
    };

    rsx! {
        Toast {
            id: props.id,
            index: props.index,
            title: props.title,
            description: props.description,
            toast_type: props.toast_type,
            on_close: props.on_close,
            permanent: props.permanent,
            // Timing is handled above; keep the primitive from removing the toast itself.
            duration: None,
            class: format!("{TOAST_BASE} {motion}"),
            "data-slot": "toast",
            attributes: props.attributes,
            ToastContent {
                ToastTitle {}
                ToastDescription {}
            }
            button {
                r#type: "button",
                aria_label: "chiudi notifica",
                class: "-mr-1 shrink-0 cursor-pointer self-start rounded-full border-none bg-transparent p-1 text-lg leading-none text-current opacity-50 transition hover:bg-primary/10 hover:opacity-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent/40",
                onclick: move |_| close.call(()),
                "×"
            }
        }
    }
}

#[component]
fn ToastContent(props: ToastContentProps) -> Element {
    rsx! {
        toast::ToastContent {
            class: "flex min-w-0 flex-1 flex-col gap-1",
            "data-slot": "toast-content",
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
fn ToastTitle(props: ToastTitleProps) -> Element {
    rsx! {
        toast::ToastTitle {
            class: "text-sm font-semibold leading-none text-primary group-data-[type=success]/toast:text-secondary group-data-[type=error]/toast:text-red-800 group-data-[type=warning]/toast:text-accent",
            "data-slot": "toast-title",
            attributes: props.attributes,
            children: props.children,
        }
    }
}

#[component]
fn ToastDescription(props: ToastDescriptionProps) -> Element {
    rsx! {
        toast::ToastDescription {
            class: "text-sm leading-snug text-primary/75 group-data-[type=error]/toast:text-red-900/80",
            "data-slot": "toast-description",
            attributes: props.attributes,
            children: props.children,
        }
    }
}

#[component]
pub fn ToastProvider(
    #[props(default = ReadSignal::new(Signal::new(Some(Duration::from_secs(5)))))]
    default_duration: ReadSignal<Option<Duration>>,
    #[props(default = ReadSignal::new(Signal::new(10)))] max_toasts: ReadSignal<usize>,
    #[props(default)] render_toast: Option<Callback<toast::ToastPropsWithOwner, Element>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let render_toast = render_toast.unwrap_or_else(|| {
        Callback::new(|p: toast::ToastPropsWithOwner| rsx! { StyledToast { ..p } })
    });

    rsx! {
        toast::ToastProvider {
            class: "fixed right-5 bottom-5 z-[9999] max-w-[350px] [&_ol]:m-0 [&_ol]:flex [&_ol]:flex-col-reverse [&_ol]:gap-3 [&_ol]:p-0 [&_li]:flex",
            "data-slot": "toast-container",
            default_duration,
            max_toasts,
            render_toast,
            attributes,
            {children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus_primitives::toast::{use_toast, ToastOptions};

    #[component]
    fn TriggerToast() -> Element {
        let toast_api = use_toast();
        use_hook(move || {
            toast_api.success(
                "Saved".to_string(),
                ToastOptions::new()
                    .description("Everything synced")
                    .permanent(true),
            );
        });

        rsx! {}
    }

    #[test]
    fn styled_toast_preserves_primitive_fallback_children() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                ToastProvider {
                    TriggerToast {}
                }
            }
        });
        dom.rebuild_in_place();
        dom.mark_all_dirty();
        dom.render_immediate_to_vec();
        let html = dioxus_ssr::render(&dom);

        assert!(html.contains("Saved"));
        assert!(html.contains("Everything synced"));
        assert!(html.contains('\u{00d7}') || html.contains("&#215;") || html.contains("&times;"));
    }
}
