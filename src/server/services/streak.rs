//! How many days in a row a user has kept their diary.
//!
//! - A day counts once something was tracked on it: a weight or at least one
//!   entry. A day holding only the calorie target doesn't.
//! - It must be tracked by the end of [`GRACE_DAYS`] days after its date:
//!   Monday can still be filled in on Wednesday, not on Thursday. The diary
//!   stays editable for any date, but a late day never counts, so the streak
//!   can't be won back by backfilling.
//! - The streak is the run of consecutive counted days ending at the most
//!   recent one. When a run is followed by a gap that can still be filled in
//!   time, the streak is *at risk*: the gap may still join the runs together.
//! - Once a gap can no longer be filled, the runs before it are lost for good.
//!
//! Dates follow the server's clock.

use chrono::{Days, NaiveDate};
use std::collections::BTreeSet;

pub const GRACE_DAYS: u64 = 2;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Streak {
    pub days: i32,
    pub at_risk: bool,
}

/// `tracked` pairs each date with the date it was first tracked on.
pub fn compute(
    today: NaiveDate,
    tracked: impl IntoIterator<Item = (NaiveDate, NaiveDate)>,
) -> Streak {
    let counted: BTreeSet<NaiveDate> = tracked
        .into_iter()
        .filter(|&(date, tracked_on)| date <= today && tracked_on <= date + Days::new(GRACE_DAYS))
        .map(|(date, _)| date)
        .collect();

    let first_fillable = today - Days::new(GRACE_DAYS);
    let last_bridgeable = first_fillable - Days::new(1);

    let Some(&latest) = counted.last() else {
        return Streak::default();
    };
    if latest < last_bridgeable {
        return Streak::default();
    }

    let days = latest
        .iter_days()
        .rev()
        .take_while(|date| counted.contains(date))
        .count();

    let at_risk = first_fillable
        .iter_days()
        .take_while(|&date| date < today)
        .any(|gap| {
            !counted.contains(&gap) && counted.range(last_bridgeable..gap).next().is_some()
        });

    Streak {
        days: i32::try_from(days).unwrap_or(i32::MAX),
        at_risk,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 9, day).unwrap()
    }

    /// Days tracked on their own date.
    fn on_time(days: impl IntoIterator<Item = u32>) -> Vec<(NaiveDate, NaiveDate)> {
        days.into_iter().map(|day| (date(day), date(day))).collect()
    }

    fn streak(days: i32, at_risk: bool) -> Streak {
        Streak { days, at_risk }
    }

    #[test]
    fn nothing_tracked() {
        assert_eq!(compute(date(20), []), streak(0, false));
    }

    #[test]
    fn counts_consecutive_days_up_to_today() {
        assert_eq!(compute(date(20), on_time(15..=20)), streak(6, false));
    }

    #[test]
    fn today_not_tracked_yet_is_not_a_risk() {
        assert_eq!(compute(date(21), on_time(15..=20)), streak(6, false));
    }

    #[test]
    fn missing_yesterday_is_at_risk() {
        assert_eq!(compute(date(22), on_time(15..=20)), streak(6, true));
        assert_eq!(compute(date(23), on_time(15..=20)), streak(6, true));
    }

    #[test]
    fn gap_past_the_grace_period_resets() {
        assert_eq!(compute(date(24), on_time(15..=20)), streak(0, false));
    }

    #[test]
    fn late_fill_within_grace_keeps_the_streak() {
        let mut tracked = on_time((15..=20).chain([22]));
        tracked.push((date(21), date(23)));

        assert_eq!(compute(date(23), tracked), streak(8, false));
    }

    #[test]
    fn late_fill_past_grace_does_not_count() {
        let mut tracked = on_time(15..=20);
        tracked.push((date(21), date(24)));

        assert_eq!(compute(date(24), tracked), streak(0, false));
    }

    #[test]
    fn open_gap_counts_the_latest_run_only() {
        assert_eq!(
            compute(date(22), on_time((15..=20).chain([22]))),
            streak(1, true)
        );
        assert_eq!(
            compute(date(24), on_time((15..=20).chain([22, 23]))),
            streak(2, false)
        );
    }

    #[test]
    fn filling_the_gap_joins_the_runs() {
        let mut tracked = on_time((15..=20).chain([22]));
        tracked.push((date(21), date(22)));

        assert_eq!(compute(date(22), tracked), streak(8, false));
    }

    #[test]
    fn future_days_are_ignored() {
        assert_eq!(compute(date(20), on_time(18..=25)), streak(3, false));
    }

    #[test]
    fn first_day_alone_is_not_at_risk() {
        assert_eq!(compute(date(20), on_time([20])), streak(1, false));
    }
}
