//! Drawing of the delete confirmation dialog.

use eframe::egui::{self, Id, Modal, RichText};

use crate::ui::UiAction;
use crate::ui::views::ERROR_COLOR;

/// Width of the dialog, in logical pixels.
const DIALOG_WIDTH: f32 = 340.0;

/// Draws a modal asking to confirm the deletion of the activity `name`.
///
/// Returns [`UiAction::ConfirmDelete`] or [`UiAction::CancelDelete`] once the
/// user decided; pressing Escape or clicking outside cancels.
#[must_use]
pub fn draw_delete_dialog(context: &egui::Context, name: &str) -> Option<UiAction> {
    let mut action = None;
    let display_name = if name.trim().is_empty() {
        "this new activity"
    } else {
        name
    };

    let modal = Modal::new(Id::unique("delete_activity_dialog")).show(context, |ui| {
        ui.set_width(DIALOG_WIDTH);
        ui.heading("Delete this activity?");
        ui.add_space(6.0);
        ui.label(format!(
            "\"{display_name}\" and its folder will be permanently deleted. This cannot be undone."
        ));
        ui.add_space(12.0);
        ui.horizontal(|ui| {
            if ui
                .button(RichText::new("Delete").color(ERROR_COLOR))
                .clicked()
            {
                action = Some(UiAction::ConfirmDelete);
            }
            if ui.button("Cancel").clicked() {
                action = Some(UiAction::CancelDelete);
            }
        });
    });

    if action.is_none() && modal.should_close() {
        action = Some(UiAction::CancelDelete);
    }
    action
}
