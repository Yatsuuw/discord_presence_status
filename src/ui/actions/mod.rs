//! Application of user actions: the only UI code with side effects.
//!
//! Each action lives in its own file; [`apply_action`] routes a
//! [`UiAction`](crate::ui::UiAction) to the matching function.

mod apply_action;
mod create_new_activity;
mod delete_selected_activity;
mod open_activities_folder;
mod pick_image;
mod poll_presence_status;
mod reload_activities;
mod save_draft;
mod select_activity;
mod show_error;
mod show_success;
mod start_selected_activity;
mod stop_presence;

pub use apply_action::apply_action;
pub use create_new_activity::create_new_activity;
pub use delete_selected_activity::delete_selected_activity;
pub use open_activities_folder::open_activities_folder;
pub use pick_image::pick_image;
pub use poll_presence_status::poll_presence_status;
pub use reload_activities::reload_activities;
pub use save_draft::save_draft;
pub use select_activity::select_activity;
pub use show_error::show_error;
pub use show_success::show_success;
pub use start_selected_activity::start_selected_activity;
pub use stop_presence::stop_presence;
