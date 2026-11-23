//! The desktop user interface, built with `egui`/`eframe`.
//!
//! The interface follows a simple one-way flow on every frame:
//!
//! 1. [`views`] draw the current [`state`] and return the [`UiAction`]s the
//!    user triggered (clicks, confirmations…), editing only form fields;
//! 2. [`actions`] apply those actions: they are the only code allowed to
//!    touch the disk or talk to the presence worker.
//!
//! Keeping drawing and side effects apart makes both easy to follow. The
//! [`tray`] icon follows the same flow: its clicks become `UiAction`s.

pub mod actions;
pub mod state;
pub mod tray;
pub mod views;

mod build_native_options;
mod build_presence_app;
mod create_presence_app;
mod load_window_icon;
mod presence_app_impl;
mod repaint_window;
mod run_app;
mod run_window;
mod ui_action;
mod wait_in_tray;
mod window_session;

pub use build_native_options::build_native_options;
pub use build_presence_app::build_presence_app;
pub use create_presence_app::create_presence_app;
pub use load_window_icon::load_window_icon;
pub use repaint_window::repaint_window;
pub use run_app::run_app;
pub use run_window::run_window;
pub use ui_action::UiAction;
pub use wait_in_tray::wait_in_tray;
pub use window_session::WindowSession;
