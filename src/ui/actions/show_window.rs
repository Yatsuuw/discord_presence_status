//! Bringing the open window to the front.

use eframe::egui::{self, ViewportCommand};

use crate::ui::state::PresenceApp;

/// Restores and focuses the window when the tray icon is clicked while the
/// window is open (for example minimised behind other windows).
///
/// When the window is closed, [`run_app`](crate::ui::run_app) creates a new
/// one instead.
pub fn show_window(app: &mut PresenceApp, context: &egui::Context) {
    app.window_exit = None;
    context.send_viewport_cmd(ViewportCommand::Visible(true));
    context.send_viewport_cmd(ViewportCommand::Minimized(false));
    context.send_viewport_cmd(ViewportCommand::Focus);
}
