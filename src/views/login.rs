use dioxus::prelude::*;

use crate::components::ui::{button::Button, input::Input, label::Label, separator::Separator};

#[component]
pub fn Login() -> Element {
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
                div {
                    class: "max-w-md",
                    p {
                        class: "text-accent text-xs font-semibold mb-2",
                        "BENTORNATO"
                    }
                    h1 {
                        class: "font-heading text-3xl mb-3",
                        "Accedi al tuo diario"
                    }
                    p {
                        class: "mb-8",
                        "Continua a monitorare la tua dieta e quella del tuo gruppo."
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
                            "Accedi"
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
                        "Non hai un account? "
                        Link {
                            class: "text-accent hover:underline",
                            to: "#",
                            "Registrati"
                        }
                    }
                }
            }
        }
    }
}
