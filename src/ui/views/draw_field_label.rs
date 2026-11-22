//! Drawing of a form label followed by its help icon.

use eframe::egui::{self, Response};

use crate::ui::views::draw_help_marker;

/// Draws `text` and a "(?)" icon explaining the field, side by side.
///
/// Returns the label response, so the field can be linked to it for screen
/// readers with [`Response::labelled_by`].
pub fn draw_field_label(ui: &mut egui::Ui, text: &str, help: &str) -> Response {
    ui.horizontal(|ui| {
        let label = ui.label(text).on_hover_text(help);
        draw_help_marker(ui, help);
        label
    })
    .inner
}
