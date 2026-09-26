use super::feedback::or_toast;
use super::stats::{day_progress, weight_delta, weight_starting_month};
use crate::api::{day, entry, users};
use crate::schema::{day::DaySchema, entry::EntrySchema, user::UserSchema};
use chrono::NaiveDate;
use dioxus_primitives::toast::Toasts;

#[derive(Default)]
pub struct HomeData {
    pub community: Vec<CommunityMember>,
    pub days: Vec<DaySchema>,
    pub entries: Vec<EntrySchema>,
}

/// A community member paired with the stats `UserRow` shows for them. Their
/// `days`/`entries` are public read data, fetched the same way regardless of
/// whether they belong to the signed-in user.
#[derive(Clone, PartialEq)]
pub struct CommunityMember {
    pub user: UserSchema,
    pub weight_delta: f32,
    pub starting_month: Option<String>,
    pub calories: i32,
    pub target_calories: i32,
}

impl CommunityMember {
    fn new(
        user: UserSchema,
        today: NaiveDate,
        days: &[DaySchema],
        entries: &[EntrySchema],
    ) -> Self {
        let (calories, target_calories) = day_progress(today, days, entries);
        Self {
            user,
            weight_delta: weight_delta(days),
            starting_month: weight_starting_month(days),
            calories,
            target_calories,
        }
    }
}

pub async fn load_home_data(current_user_id: i32, today: NaiveDate, toast_api: Toasts) -> HomeData {
    let days = or_toast(day::list().await, toast_api);
    let entries = or_toast(entry::list().await, toast_api);
    let users = or_toast(users::list().await, toast_api);

    let mut community = Vec::with_capacity(users.len());

    for member in users {
        // The signed-in user's own days/entries are already fetched above;
        // everyone else's are public read data, fetched on-demand through the
        // community-scoped endpoints.
        let community_member = if member.id == current_user_id {
            CommunityMember::new(member, today, &days, &entries)
        } else {
            let member_days = or_toast(day::list_for_user(member.name.clone()).await, toast_api);
            let member_entries =
                or_toast(entry::list_for_user(member.name.clone()).await, toast_api);
            CommunityMember::new(member, today, &member_days, &member_entries)
        };

        community.push(community_member);
    }

    HomeData {
        community,
        days,
        entries,
    }
}
