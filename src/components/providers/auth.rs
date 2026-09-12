use crate::api::auth::logout;
use crate::api::auth::me;
use crate::api::auth::refresh;
use crate::schema::user::UserSchema;
use dioxus::prelude::*;

#[derive(Clone, Copy)]
pub struct AuthState {
    pub user: Signal<Option<UserSchema>>,
    pub is_loading: Signal<bool>,
}

impl AuthState {
    pub async fn get_session(mut self) {
        self.is_loading.set(true);

        if refresh().await.is_err() {
            let _ = logout().await;
            self.clear();
            return;
        }

        match me().await {
            Ok(user) => self.user.set(Some(user)),
            Err(_) => self.user.set(None),
        }

        self.is_loading.set(false);
    }

    pub fn clear(mut self) {
        self.user.set(None);
        self.is_loading.set(false);
    }
}

#[component]
pub fn AuthProvider(children: Element) -> Element {
    let user = use_signal(|| None::<UserSchema>);
    let is_loading = use_signal(|| true);
    let auth = AuthState { user, is_loading };

    use_context_provider(|| auth);

    use_effect(move || {
        spawn(async move {
            auth.get_session().await;
        });
    });

    rsx! { {children} }
}

pub fn use_auth() -> AuthState {
    use_context::<AuthState>()
}
