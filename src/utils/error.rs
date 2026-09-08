use dioxus::prelude::ServerFnError;

pub fn error_message(err: &ServerFnError) -> String {
    match err {
        ServerFnError::ServerError { message, .. } => message.clone(),
        ServerFnError::Request(_) => "Impossibile raggiungere il server".to_string(),
        other => other.to_string(),
    }
}
