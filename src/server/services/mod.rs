pub mod auth;
pub mod day;
pub mod diary;
pub mod entry;
pub mod stats;
pub mod streak;
pub mod users;

/// A blank search string means "no search", not "match an empty string".
fn non_blank(text: Option<String>) -> Option<String> {
    text.map(|text| text.trim().to_string())
        .filter(|text| !text.is_empty())
}
