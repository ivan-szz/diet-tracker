//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component and an Echo component for fullstack apps to be used in our app.

mod user_row;
pub use user_row::UserRow;

mod entry_row;
pub use entry_row::EntryRow;

mod day_block;
pub use day_block::DayBlock;

pub mod ui;
