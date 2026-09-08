use dioxus::prelude::*;

use views::{AuthLayout, Home, Login, ProvidersLayout, Register};

mod api;
mod components;
mod schema;
#[cfg(feature = "server")]
mod server;
mod utils;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(ProvidersLayout)]
        #[route("/")]
        Home {},
        #[layout(AuthLayout)]
            #[route("/login")]
            Login,
            #[route("/register")]
            Register,
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

/// Client entrypoint (web/desktop/mobile): no database, just the UI.
#[cfg(not(feature = "server"))]
fn main() {
    dioxus::launch(App);
}

/// Server entrypoint.
///
/// `dioxus::serve` replaces `dioxus::launch` when a custom axum router is
/// needed. The closure is re-run on every hot-patch, so everything that must
/// happen exactly once lives behind the `OnceLock` in `db::init`.
#[cfg(feature = "server")]
fn main() {
    use dioxus::server::axum::Extension;

    // `dx serve` does not read the .env, so without this DATABASE_URL is
    // missing at runtime.
    dotenvy::dotenv().ok();

    dioxus::serve(|| async move {
        let pool = server::db::init().await?;

        // The Extension layer puts the pool into the extensions of *every*
        // request: both the server function calls and the SSR page render.
        Ok(dioxus::server::router(App).layer(Extension(pool)))
    })
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link {
            rel: "preconnect",
            href: "https://fonts.googleapis.com",
        }
        document::Link {
            rel: "preconnect",
            href: "https://fonts.gstatic.com",
            crossorigin: "anonymous",
        }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Caprasimo&family=Figtree:ital,wght@0,300..900;1,300..900&display=swap",
        }

        // The router component renders the route enum we defined above. It will handle synchronization of the URL and render
        // the layouts and components for the active route.
        Router::<Route> {}
    }
}
