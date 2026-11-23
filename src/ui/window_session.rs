//! Definition of [`WindowSession`], one opening of the main window.

use eframe::egui;

use crate::ui::keyboard::apply_alt_code_input;
use crate::ui::state::PresenceApp;

/// The main window, borrowing the application state for as long as it is
/// open.
///
/// The state outlives the window: closing the window to the tray destroys
/// the session but keeps the activities, the tray icon and the Discord
/// connection.
pub struct WindowSession<'app> {
    /// The application state shown by this window.
    pub app: &'app mut PresenceApp,
}

impl eframe::App for WindowSession<'_> {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        eframe::App::ui(self.app, ui, frame);
    }

    fn raw_input_hook(&mut self, _context: &egui::Context, raw_input: &mut egui::RawInput) {
        if let Ok(mut input) = self.app.alt_code_input.lock() {
            apply_alt_code_input(&mut input, raw_input);
        }
    }
}
