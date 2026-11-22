//! Drawing of the action buttons of the activity editor.

use eframe::egui::{self, Align, Layout};

use crate::i18n::Translations;
use crate::ui::UiAction;

/// Draws the Save / Start / Stop / Delete buttons.
///
/// The Start button becomes Stop when the activity is displayed on Discord.
pub fn draw_editor_buttons(
    ui: &mut egui::Ui,
    is_active: bool,
    texts: &Translations,
) -> Option<UiAction> {
    let mut action = None;

    ui.horizontal(|ui| {
        let save_help = if is_active {
            texts.save_help_active
        } else {
            texts.save_help
        };
        if ui
            .button(texts.save_button)
            .on_hover_text(save_help)
            .clicked()
        {
            action = Some(UiAction::Save);
        }

        if is_active {
            if ui
                .button(texts.stop_button)
                .on_hover_text(texts.stop_help)
                .clicked()
            {
                action = Some(UiAction::StopPresence);
            }
        } else if ui
            .button(texts.start_button)
            .on_hover_text(texts.start_help)
            .clicked()
        {
            action = Some(UiAction::StartPresence);
        }

        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if ui
                .button(texts.delete_button)
                .on_hover_text(texts.delete_help)
                .clicked()
            {
                action = Some(UiAction::RequestDelete);
            }
        });
    });

    action
}
