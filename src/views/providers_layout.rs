use crate::components::providers::auth::AuthProvider;
use crate::components::ui::toast::ToastProvider;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn ProvidersLayout() -> Element {
    rsx! {
        ToastProvider {
            AuthProvider {
                Outlet::<Route> {}
            }
        }
    }
}
