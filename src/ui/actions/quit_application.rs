//! Closing of the application from the tray.

use eframe::egui::{self, ViewportCommand};

use crate::ui::state::PresenceApp;

/// Closes the application, even when it would otherwise hide to the tray.
///
/// The window close then runs `on_exit`, which clears the Discord presence
/// with [`shutdown_presence_worker`](crate::presence::shutdown_presence_worker).
pub fn quit_application(app: &mut PresenceApp, context: &egui::Context) {
    app.quit_requested = true;
    context.send_viewport_cmd(ViewportCommand::Close);
    context.request_repaint();
}
