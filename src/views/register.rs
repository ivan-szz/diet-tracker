use dioxus::prelude::*;

use crate::{Route, components::ui::{button::Button, input::Input, label::Label, separator::Separator}};

#[component]
pub fn Register() -> Element {
    rsx! {
        div {
            class: "max-w-md w-full",
            p {
                class: "text-accent text-xs font-semibold mb-2",
                "BENVENUTO"
            }
            h1 {
                class: "font-heading text-3xl mb-6",
                "Registrati e inizia"
                br {  }
                "il tuo diario"
            }
            form {
                class: "flex flex-col gap-5",
                div {
                    class: "space-y-2",
                    Label {
                        html_for: "name",
                        "Nome utente"
                    }
                    Input {
                        id: "name",
                        name: "name",
                        placeholder: "Come ti chiami"
                    }
                }
                div {
                    class: "space-y-2",
                    Label {
                        html_for: "password",
                        "Password"
                    }
                    Input {
                        id: "password",
                        name: "password",
                        type: "password",
                        placeholder: "••••••••"
                    }
                }
                Button {
                    class: "font-heading",
                    "Registrati"
                }
            }
            div {
                class: "flex items-center text-xs gap-4",
                Separator {
                    class: "my-10"
                }
                p { "OPPURE" }
                Separator {
                    class: "my-10"
                }
            }
            p {
                class: "text-center",
                "Hai già un account? "
                Link {
                    class: "text-accent hover:underline",
                    to: Route::Login {},
                    "Accedi"
                }
            }
        }
    }
}
