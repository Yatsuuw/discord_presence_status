//! Hiding of the main window to the tray.

use eframe::egui::{self, ViewportCommand};

use crate::ui::state::PresenceApp;

/// Hides the main window, keeping the application (and the Discord
/// presence) running behind the tray icon.
///
/// Also cancels a pending close request, so closing the window hides it
/// instead of quitting.
pub fn hide_to_tray(app: &mut PresenceApp, context: &egui::Context) {
    context.send_viewport_cmd(ViewportCommand::CancelClose);
    context.send_viewport_cmd(ViewportCommand::Visible(false));
    app.window_hidden = true;
}
