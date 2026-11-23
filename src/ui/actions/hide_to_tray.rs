//! Hiding of the application behind its tray icon.

use eframe::egui::{self, ViewportCommand};

use crate::ui::state::{PresenceApp, WindowExit};

/// Closes the main window while keeping the presence running behind the
/// tray icon.
///
/// The window is really destroyed, not just hidden: its graphics context is
/// what uses most of the memory. [`run_app`](crate::ui::run_app) recreates
/// it when the user clicks the tray icon.
pub fn hide_to_tray(app: &mut PresenceApp, context: &egui::Context) {
    app.window_exit = Some(WindowExit::HideToTray);
    context.send_viewport_cmd(ViewportCommand::Close);
}
