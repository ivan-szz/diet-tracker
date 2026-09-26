use crate::components::home::{
    load_home_data, CommunityCard, FoodDiary, HomeData, HomeHeader, TrendCard, WeightGoalCard,
};
use crate::components::providers::auth::use_auth;
use crate::components::ui::separator::Separator;
use chrono::Local;
use dioxus::prelude::*;
use dioxus_primitives::toast::{use_toast, ToastOptions};
use std::time::Duration;

const PAGE_CLASS: &str = "p-8 pt-20 flex flex-col gap-7 max-w-5xl mx-auto";

#[component]
pub fn Home() -> Element {
    let session = use_auth();
    let toast_api = use_toast();
    let navigator = use_navigator();
    let today = Local::now().date_naive();

    // Bumped after every write, so each section's data refetches.
    let mut revision = use_signal(|| 0u32);
    let home_data_resource = use_resource(move || {
        revision();
        let is_signed_in = session.user.read().is_some();

        async move {
            if is_signed_in {
                load_home_data(today, toast_api).await
            } else {
                HomeData::default()
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
    let Some(HomeData {
        summary: Some(summary),
        community,
        trend,
    }) = home_data_state.as_ref()
    else {
        return rsx! { div { class: PAGE_CLASS } };
    };

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
                today,
                on_change: refresh_home_data,
            }
            Separator {
                class: "opacity-20"
            }
            CommunityCard {
                members: community.clone(),
                current_user_id: user.id,
            }
            if let Some((starting_kg, target_kg, percent, remaining_kg)) = weight_goal {
                WeightGoalCard { starting_kg, target_kg, percent, remaining_kg }
            }
            TrendCard {
                points: trend.clone(),
            }
            FoodDiary {
                user_name: user.name.clone(),
                today,
                total_days: summary.recorded_days,
                revision,
                on_change: refresh_home_data,
            }
        }
    }
}
