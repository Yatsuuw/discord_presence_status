//! Drawing of the delete confirmation dialog.

use eframe::egui::{self, Id, Modal, RichText};

use crate::i18n::{Translations, fill_placeholders};
use crate::ui::UiAction;
use crate::ui::views::ERROR_COLOR;

/// Width of the dialog, in logical pixels.
const DIALOG_WIDTH: f32 = 340.0;

/// Draws a modal asking to confirm the deletion of the activity `name`.
///
/// Returns [`UiAction::ConfirmDelete`] or [`UiAction::CancelDelete`] once the
/// user decided; pressing Escape or clicking outside cancels.
#[must_use]
pub fn draw_delete_dialog(
    context: &egui::Context,
    name: &str,
    texts: &Translations,
) -> Option<UiAction> {
    let mut action = None;
    let display_name = if name.trim().is_empty() {
        texts.delete_dialog_unnamed
    } else {
        name
    };

    let modal = Modal::new(Id::unique("delete_activity_dialog")).show(context, |ui| {
        ui.set_width(DIALOG_WIDTH);
        ui.heading(texts.delete_dialog_title);
        ui.add_space(6.0);
        ui.label(fill_placeholders(
            texts.delete_dialog_message,
            &[("name", display_name)],
        ));
        ui.add_space(12.0);
        ui.horizontal(|ui| {
            if ui
                .button(RichText::new(texts.delete_button).color(ERROR_COLOR))
                .clicked()
            {
                action = Some(UiAction::ConfirmDelete);
            }
            if ui.button(texts.cancel_button).clicked() {
                action = Some(UiAction::CancelDelete);
            }
        });
    });

    if action.is_none() && modal.should_close() {
        action = Some(UiAction::CancelDelete);
    }
    action
}
