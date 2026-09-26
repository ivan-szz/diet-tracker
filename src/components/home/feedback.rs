use crate::utils::error::error_message;
use dioxus::prelude::*;
use dioxus_primitives::toast::{ToastOptions, Toasts};
use serde::de::DeserializeOwned;
use std::time::Duration;
use validator::Validate;

pub fn toast_error(toast_api: Toasts, message: impl ToString) {
    toast_api.error(
        "Errore".to_string(),
        ToastOptions::new()
            .description(message)
            .duration(Duration::from_secs(20)),
    );
}

pub fn toast_success(toast_api: Toasts, message: impl ToString) {
    toast_api.success(
        "Fatto".to_string(),
        ToastOptions::new()
            .description(message)
            .duration(Duration::from_secs(20)),
    );
}

/// Unwraps a server call's result, toasting the error and falling back to an
/// empty list so one failed fetch doesn't blank the whole page.
pub fn or_toast<T>(result: Result<Vec<T>, ServerFnError>, toast_api: Toasts) -> Vec<T> {
    result.unwrap_or_else(|error| {
        toast_error(toast_api, error_message(&error));
        vec![]
    })
}

/// Parses and validates a submitted form, toasting every problem found.
pub fn parse_form<T: DeserializeOwned + Validate>(
    event: &Event<FormData>,
    toast_api: Toasts,
) -> Option<T> {
    let payload: T = match event.data().parsed_values() {
        Ok(payload) => payload,
        Err(err) => {
            toast_error(toast_api, err);
            return None;
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
                toast_error(toast_api, msg);
            }
        }
        return None;
    }

    Some(payload)
}
