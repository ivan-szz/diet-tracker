use crate::components::home::{
    load_home_data, CommunityCard, FoodDiary, HomeData, HomeHeader, TrendCard, WeightGoalCard,
};
use crate::components::providers::auth::use_auth;
use crate::components::ui::separator::Separator;
use crate::Route;
use chrono::Local;
use dioxus::prelude::*;
use dioxus_icons::lucide::ArrowLeft;
use dioxus_primitives::toast::{use_toast, ToastOptions};
use std::time::Duration;

const PAGE_CLASS: &str = "p-8 pt-20 flex flex-col gap-7 max-w-5xl mx-auto";

#[component]
pub fn Home() -> Element {
    rsx! {
        HomePage { viewed_user_name: None }
    }
}

/// Another user's progress, read-only. Keyed on the name so switching
/// profiles starts from fresh data and default diary filters.
#[component]
pub fn UserProfile(user_name: String) -> Element {
    rsx! {
        HomePage { key: "{user_name}", viewed_user_name: Some(user_name.clone()) }
    }
}

/// `viewed_user_name` is whose data the page shows; `None` means the
/// signed-in user.
#[component]
fn HomePage(viewed_user_name: Option<String>) -> Element {
    let session = use_auth();
    let toast_api = use_toast();
    let navigator = use_navigator();
    let today = Local::now().date_naive();

    // Bumped after every write, so each section's data refetches.
    let mut revision = use_signal(|| 0u32);
    let loaded_user_name = viewed_user_name.clone();
    let home_data_resource = use_resource(move || {
        revision();
        let is_signed_in = session.user.read().is_some();
        let user_name = loaded_user_name.clone();

        async move {
            if is_signed_in {
                Some(load_home_data(user_name, today, toast_api).await)
            } else {
                None
            }
        }
    });
    let refresh_home_data = use_callback(move |_| *revision.write() += 1);

    if *session.is_loading.read() {
        return rsx! { div { class: PAGE_CLASS } };
    }

    let user_state = session.user.read();
    let Some(user) = user_state.as_ref() else {
        toast_api.error(
            "Error".to_string(),
            ToastOptions::new()
                .description("You need to be logged in")
                .duration(Duration::from_secs(20)),
        );
        navigator.push("/login");
        return rsx! {};
    };

    let home_data_state = home_data_resource.read();
    let HomeData {
        summary,
        community,
        trend,
    } = match home_data_state.as_ref() {
        Some(Some(Ok(home_data))) => home_data,
        Some(Some(Err(message))) => {
            return rsx! {
                div {
                    class: PAGE_CLASS,
                    p {
                        class: "font-heading text-3xl",
                        "{message}"
                    }
                    Link {
                        class: "inline-flex items-center gap-1 text-sm text-accent hover:underline",
                        to: Route::Home {},
                        ArrowLeft {
                            size: "1em"
                        }
                        "Torna ai tuoi progressi"
                    }
                }
            };
        }
        _ => return rsx! { div { class: PAGE_CLASS } },
    };

    let is_me = summary.user.id == user.id;
    let weight = &summary.weight;
    let weight_goal = match (
        weight.starting_kg,
        weight.target_kg,
        weight.goal_progress_percent,
        weight.remaining_kg,
    ) {
        (Some(starting_kg), Some(target_kg), Some(percent), Some(remaining_kg)) => {
            Some((starting_kg, target_kg, percent, remaining_kg))
        }
        _ => None,
    };

    rsx! {
        div {
            class: PAGE_CLASS,
            HomeHeader {
                summary: summary.clone(),
                is_me,
                today,
                on_change: refresh_home_data,
            }
            Separator {
                class: "opacity-20"
            }
            CommunityCard {
                members: community.clone(),
                current_user_id: user.id,
                viewed_user_id: summary.user.id,
            }
            if let Some((starting_kg, target_kg, percent, remaining_kg)) = weight_goal {
                WeightGoalCard { starting_kg, target_kg, percent, remaining_kg }
            }
            TrendCard {
                points: trend.clone(),
                owner_name: (!is_me).then(|| summary.user.name.clone()),
            }
            FoodDiary {
                viewed_user_name: summary.user.name.clone(),
                is_me,
                current_user_name: user.name.clone(),
                today,
                total_days: summary.recorded_days,
                revision,
                on_change: refresh_home_data,
            }
        }
    }
}
