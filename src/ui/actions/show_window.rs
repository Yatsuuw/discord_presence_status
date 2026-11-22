//! Display of the main window from the tray.

use eframe::egui::{self, ViewportCommand};

use crate::ui::state::PresenceApp;

/// Shows the main window again, restores it if it was minimised and brings
/// it to the front.
pub fn show_window(app: &mut PresenceApp, context: &egui::Context) {
    context.send_viewport_cmd(ViewportCommand::Visible(true));
    context.send_viewport_cmd(ViewportCommand::Minimized(false));
    context.send_viewport_cmd(ViewportCommand::Focus);
    app.window_hidden = false;
}
