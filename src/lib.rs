pub mod app;
mod components;
pub mod features;
pub mod routes;
mod schema;
#[cfg(feature = "ssr")]
pub mod server;
pub mod utils;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
