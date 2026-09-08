use crate::api::auth::register;
use crate::schema::user::RegisterUserSchema;
use crate::utils::error::error_message;
use crate::{
    components::ui::{button::Button, input::Input, label::Label, separator::Separator},
    Route,
};
use dioxus::prelude::*;
use dioxus_primitives::toast::{use_toast, ToastOptions};
use std::time::Duration;
use validator::Validate;
use crate::components::providers::auth::use_auth;

#[component]
pub fn Register() -> Element {
    let toast_api = use_toast();
    let navigator = use_navigator();
    let session = use_auth();

    let handle_submit = move |e: Event<FormData>| async move {
        e.prevent_default();
        let payload: RegisterUserSchema = match e.data().parsed_values() {
            Ok(v) => v,
            Err(e) => {
                toast_api.error(
                    "Errore".to_string(),
                    ToastOptions::new()
                        .description(e.to_string())
                        .duration(Duration::from_secs(20)),
                );
                return;
            }
        };

        if let Err(errors) = payload.validate() {
            for (field, errs) in errors.field_errors() {
                for err in errs {
                    let msg = err
                        .message
                        .as_ref()
                        .map(|m| m.to_string())
                        .unwrap_or_else(|| format!("{}: {}", field, err.code));
                    toast_api.error(
                        "Errore".to_string(),
                        ToastOptions::new()
                            .description(msg)
                            .duration(Duration::from_secs(20)),
                    );
                }
            }
            return;
        }

        if let Err(err) = register(payload).await {
            toast_api.error(
                "Errore".to_string(),
                ToastOptions::new()
                    .description(error_message(&err))
                    .duration(Duration::from_secs(20)),
            );
            return;
        }

        toast_api.success(
            "Completato".to_string(),
            ToastOptions::new()
                .description("Ti sei registrato!")
                .duration(Duration::from_secs(20)),
        );
        session.get_session().await;
        navigator.push("/");
    };

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
                onsubmit: move |e| handle_submit(e),
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
