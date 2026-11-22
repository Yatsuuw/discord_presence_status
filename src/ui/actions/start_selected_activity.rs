//! Display of the edited activity on Discord.

use eframe::egui;

use crate::ui::actions::{display_activity, save_draft};
use crate::ui::state::PresenceApp;

/// Saves the activity open in the editor, then asks Discord to display it.
///
/// Discord's elapsed timer starts now. Any previously displayed activity is
/// replaced.
pub fn start_selected_activity(app: &mut PresenceApp, context: &egui::Context) {
    if let Some(stored) = save_draft(app, context) {
        display_activity(app, stored);
    }
}
