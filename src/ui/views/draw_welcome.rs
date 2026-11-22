//! Drawing of the placeholder shown when no activity is open.

use eframe::egui;

use crate::ui::UiAction;

/// Draws a short introduction and a button to create an activity.
pub fn draw_welcome(ui: &mut egui::Ui) -> Option<UiAction> {
    let mut action = None;
    ui.vertical_centered(|ui| {
        ui.add_space(ui.available_height() / 3.0);
        ui.heading("Show the world what you are up to");
        ui.add_space(6.0);
        ui.label("Select an activity on the left, or create a new one.");
        ui.add_space(12.0);
        if ui.button("+ New activity").clicked() {
            action = Some(UiAction::NewActivity);
        }
    });
    action
}
