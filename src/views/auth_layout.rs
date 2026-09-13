use crate::components::providers::auth::use_auth;
use crate::Route;
use dioxus::prelude::*;
use dioxus_primitives::toast::{use_toast, ToastOptions};
use std::time::Duration;

#[component]
pub fn AuthLayout() -> Element {
    let toast_api = use_toast();
    let navigator = use_navigator();
    let session = use_auth();

    if session.user.as_ref().is_some() {
        toast_api.info(
            "Connesso".to_string(),
            ToastOptions::new()
                .description("Sei già connesso")
                .duration(Duration::from_secs(20)),
        );
        navigator.push("/");
        return rsx! {};
    };

    rsx! {
        div {
            class: "w-full min-h-screen flex flex-col md:h-screen md:flex-row",
            div {
                class: "w-full bg-secondary flex flex-col justify-center gap-8 p-10 text-background relative overflow-hidden",
                div {
                    class: "w-full flex flex-col justify-center gap-8 relative z-10 mx-auto md:mx-0 max-w-md",
                    h2 {
                        class: "font-heading text-3xl md:text-5xl",
                        "Monitora la dieta "
                        br {
                            class: "hidden md:block"
                        }
                        "insieme al tuo "
                        br {
                            class: "hidden md:block"
                        }
                        "gruppo"
                    }
                    p {
                        class: "hidden md:block",
                        "Diario alimentare, obiettivi di peso e classifica dei progressi, condivisi con le persone che ti tengono motivato."
                    }
                }
                div {
                    class: "absolute -bottom-10 -right-10 size-52 rounded-full bg-accent/60"
                }
                div {
                    class: "absolute -top-28 -left-28 size-96 rounded-full bg-secondary-light"
                }
            }
            div {
                class: "w-full flex flex-col gap-4 justify-center items-center p-8",
                Outlet::<Route> {}
            }
        }
    }
}
