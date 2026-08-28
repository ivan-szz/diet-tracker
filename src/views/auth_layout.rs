use dioxus::prelude::*;

use crate::{Route, components::ui::{button::Button, input::Input, label::Label, separator::Separator}};

#[component]
pub fn AuthLayout() -> Element {
    rsx! {
        div {
            class: "w-full h-screen flex",
            div {
                class: "w-full bg-secondary flex flex-col justify-center gap-8 p-10 text-background relative overflow-hidden",
                div {
                    class: "w-full flex flex-col justify-center gap-8 relative z-10 max-w-md",
                    h2 {
                        class: "font-heading text-5xl",
                        "Monitora la dieta"
                        br {}
                        "insieme al tuo"
                        br {}
                        "gruppo"
                    }
                    p {
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
