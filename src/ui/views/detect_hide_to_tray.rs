//! Detection of a close or minimise request that should hide to the tray.

use eframe::egui;

use crate::ui::UiAction;
use crate::ui::state::{PresenceApp, keeps_running_in_tray};

/// Returns [`UiAction::HideToTray`] when the user closes or minimises the
/// window while the application should keep running in the tray.
///
/// Otherwise closing quits and minimising keeps its usual behaviour.
#[must_use]
pub fn detect_hide_to_tray(context: &egui::Context, app: &PresenceApp) -> Option<UiAction> {
    if !keeps_running_in_tray(app) {
        return None;
    }
    let (close_requested, minimized) = context.input(|input| {
        let viewport = input.viewport();
        (viewport.close_requested(), viewport.minimized == Some(true))
    });
    let hides = close_requested || (minimized && !app.window_hidden);
    hides.then_some(UiAction::HideToTray)
}
