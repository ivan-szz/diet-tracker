use dioxus::logger::tracing;
use dioxus::prelude::*;
use std::collections::BTreeMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("User already exists")]
    UserAlreadyExists,
    #[error(transparent)]
    Validation(#[from] validator::ValidationErrors),
    #[error(transparent)]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    Database(#[from] sqlx::Error),
    #[error(transparent)]
    Hashing(#[from] argon2::password_hash::Error),
}

impl ServerError {
    fn client_response(&self) -> (u16, &'static str, Option<serde_json::Value>) {
        match self {
            ServerError::Validation(errors) => (400, "Dati non validi", validation_details(errors)),
            ServerError::Unauthorized => (401, "Non sei autenticato", None),
            ServerError::InvalidCredentials => (401, "Nome utente o password errati", None),
            ServerError::UserAlreadyExists => (409, "Nome utente già in uso", None),
            ServerError::Jwt(_) => (401, "Sessione non valida", None),
            ServerError::Database(_) | ServerError::Hashing(_) => {
                (500, "Errore interno del server", None)
            }
        }
    }
}

impl From<ServerError> for ServerFnError {
    fn from(e: ServerError) -> Self {
        let (code, message, details) = e.client_response();
        if code >= 500 {
            error!("server error: {e:?}");
        }
        ServerFnError::ServerError {
            message: message.to_string(),
            code,
            details,
        }
    }
}

fn validation_details(errors: &validator::ValidationErrors) -> Option<serde_json::Value> {
    let fields: BTreeMap<String, Vec<String>> = errors
        .field_errors()
        .iter()
        .map(|(field, field_errors)| {
            let messages = field_errors
                .iter()
                .map(|error| {
                    error
                        .message
                        .as_ref()
                        .map(ToString::to_string)
                        .unwrap_or_else(|| error.code.to_string())
                })
                .collect();

            (field.to_string(), messages)
        })
        .collect();

    serde_json::to_value(&fields).ok()
}
