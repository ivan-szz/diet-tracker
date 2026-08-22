//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component and an Echo component for fullstack apps to be used in our app.

mod hero;
pub use hero::Hero;

mod echo;
pub use echo::Echo;
pub mod accordion;
pub mod badge;
pub mod button;
pub mod card;
pub mod checkbox;
pub mod alert_dialog;
pub mod input;
pub mod label;
pub mod progress;
pub mod separator;
pub mod switch;
