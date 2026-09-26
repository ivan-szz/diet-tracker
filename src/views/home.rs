use crate::components::home::{
    first_weight, latest_weight, load_home_data, CommunityCard, FoodDiary, HomeData, HomeHeader,
    TrendCard, WeightGoalCard,
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

    let mut home_data_resource = use_resource(move || {
        let current_user_id = session.user.read().as_ref().map(|user| user.id);

        async move {
            match current_user_id {
                Some(current_user_id) => load_home_data(current_user_id, today, toast_api).await,
                None => HomeData::default(),
            }
        }
    });
    let refresh_home_data = use_callback(move |_| home_data_resource.restart());

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
    let Some(home_data) = home_data_state.as_ref() else {
        return rsx! { div { class: PAGE_CLASS } };
    };

    let weight_goal = match (
        first_weight(&home_data.days),
        latest_weight(&home_data.days),
        user.target_weight_kg,
    ) {
        (Some((starting_kg, _)), Some(current_kg), Some(target_kg)) => {
            Some((starting_kg, current_kg, target_kg))
        }
        _ => None,
    };

    rsx! {
        div {
            class: PAGE_CLASS,
            HomeHeader {
                user: user.clone(),
                days: home_data.days.clone(),
                entries: home_data.entries.clone(),
                today,
                on_change: refresh_home_data,
            }
            Separator {
                class: "opacity-20"
            }
            CommunityCard {
                members: home_data.community.clone(),
                current_user_id: user.id,
            }
            if let Some((starting_kg, current_kg, target_kg)) = weight_goal {
                WeightGoalCard { starting_kg, current_kg, target_kg }
            }
            TrendCard {
                days: home_data.days.clone(),
                entries: home_data.entries.clone(),
                today,
            }
            FoodDiary {
                user_name: user.name.clone(),
                today,
                days: home_data.days.clone(),
                entries: home_data.entries.clone(),
                on_change: refresh_home_data,
            }
        }
    }
}
