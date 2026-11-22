//! Drawing of the placeholder shown when no activity is open.

use eframe::egui;

use crate::i18n::Translations;
use crate::ui::UiAction;

/// Draws a short introduction and a button to create an activity.
pub fn draw_welcome(ui: &mut egui::Ui, texts: &Translations) -> Option<UiAction> {
    let mut action = None;
    ui.vertical_centered(|ui| {
        ui.add_space(ui.available_height() / 3.0);
        ui.heading(texts.welcome_title);
        ui.add_space(6.0);
        ui.label(texts.welcome_text);
        ui.add_space(12.0);
        if ui
            .button(texts.new_activity_button)
            .on_hover_text(texts.new_activity_help)
            .clicked()
        {
            action = Some(UiAction::NewActivity);
        }
    });
    action
}
