use super::dates::chart_start_date;
use super::feedback::or_toast;
use crate::api::stats;
use crate::schema::stats::{
    CommunityQuerySchema, SummaryQuerySchema, TrendPointSchema, TrendQuerySchema, UserSummarySchema,
};
use crate::utils::error::error_message;
use chrono::NaiveDate;
use dioxus_primitives::toast::Toasts;

pub struct HomeData {
    pub summary: UserSummarySchema,
    pub community: Vec<UserSummarySchema>,
    pub trend: Vec<TrendPointSchema>,
}

/// `user_name` is whose summary and trend to load, `None` for the signed-in
/// user. `today` is the client's date, so "today" follows the viewer's
/// timezone rather than the server's.
///
/// Fails with the error message when the summary can't be loaded, e.g. for
/// an unknown user: the page has nothing to show without it.
pub async fn load_home_data(
    user_name: Option<String>,
    today: NaiveDate,
    toast_api: Toasts,
) -> Result<HomeData, String> {
    let summary = stats::summary(SummaryQuerySchema {
        user_name: user_name.clone(),
        date: Some(today),
    })
    .await
    .map_err(|error| error_message(&error))?;
    let community = or_toast(
        stats::community(CommunityQuerySchema { date: Some(today) }).await,
        toast_api,
    );
    let trend = or_toast(
        stats::trend(TrendQuerySchema {
            user_name,
            from: Some(chart_start_date(today)),
            to: Some(today),
        })
        .await,
        toast_api,
    );

    Ok(HomeData {
        summary,
        community,
        trend,
    })
}
