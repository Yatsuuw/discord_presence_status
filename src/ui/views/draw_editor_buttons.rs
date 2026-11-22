//! Drawing of the action buttons of the activity editor.

use eframe::egui::{self, Align, Layout};

use crate::ui::UiAction;

/// Draws the Save / Start / Stop / Delete buttons.
///
/// The Start button becomes Stop when the activity is displayed on Discord.
pub fn draw_editor_buttons(ui: &mut egui::Ui, is_active: bool) -> Option<UiAction> {
    let mut action = None;

    ui.horizontal(|ui| {
        let save_hint = if is_active {
            "Save the changes and update Discord (the timer keeps running)"
        } else {
            "Save the activity in its folder"
        };
        if ui.button("Save").on_hover_text(save_hint).clicked() {
            action = Some(UiAction::Save);
        }

        if is_active {
            if ui
                .button("Stop")
                .on_hover_text("Remove the activity from Discord")
                .clicked()
            {
                action = Some(UiAction::StopPresence);
            }
        } else if ui
            .button("Start on Discord")
            .on_hover_text("Save, then display this activity on Discord with a timer")
            .clicked()
        {
            action = Some(UiAction::StartPresence);
        }

        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if ui.button("Delete").clicked() {
                action = Some(UiAction::RequestDelete);
            }
        });
    });

    action
}
