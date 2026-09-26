use super::dates::chart_start_date;
use super::feedback::{or_toast, toast_error};
use crate::api::stats;
use crate::schema::stats::{
    CommunityQuerySchema, SummaryQuerySchema, TrendPointSchema, TrendQuerySchema, UserSummarySchema,
};
use crate::utils::error::error_message;
use chrono::NaiveDate;
use dioxus_primitives::toast::Toasts;

#[derive(Default)]
pub struct HomeData {
    /// The signed-in user's summary; `None` if it failed to load.
    pub summary: Option<UserSummarySchema>,
    pub community: Vec<UserSummarySchema>,
    pub trend: Vec<TrendPointSchema>,
}

/// `today` is the client's date, so "today" follows the viewer's timezone
/// rather than the server's.
pub async fn load_home_data(today: NaiveDate, toast_api: Toasts) -> HomeData {
    let summary = stats::summary(SummaryQuerySchema {
        user_name: None,
        date: Some(today),
    })
    .await
    .inspect_err(|error| toast_error(toast_api, error_message(error)))
    .ok();
    let community = or_toast(
        stats::community(CommunityQuerySchema { date: Some(today) }).await,
        toast_api,
    );
    let trend = or_toast(
        stats::trend(TrendQuerySchema {
            user_name: None,
            from: Some(chart_start_date(today)),
            to: Some(today),
        })
        .await,
        toast_api,
    );

    HomeData {
        summary,
        community,
        trend,
    }
}
