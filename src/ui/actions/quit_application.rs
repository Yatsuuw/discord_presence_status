//! Closing of the whole application.

use eframe::egui::{self, ViewportCommand};

use crate::ui::state::{PresenceApp, WindowExit};

/// Closes the window and stops the application; the presence is cleared
/// when [`run_app`](crate::ui::run_app) shuts the worker down.
pub fn quit_application(app: &mut PresenceApp, context: &egui::Context) {
    app.window_exit = Some(WindowExit::Quit);
    context.send_viewport_cmd(ViewportCommand::Close);
    context.request_repaint();
}
