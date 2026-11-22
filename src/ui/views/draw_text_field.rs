//! Drawing of one labelled text field of the editor form.

use eframe::egui::{self, TextEdit};

use crate::ui::views::draw_field_label;

/// Draws a label with its help icon and a single-line text field, as one
/// row of a two-column grid.
///
/// * `hint`: greyed text shown while the field is empty;
/// * `help`: explanation shown when hovering the icon, the label or the field;
/// * `max_characters`: the field refuses longer input.
///
/// The label is linked to the field so screen readers announce it.
pub fn draw_text_field(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut String,
    hint: &str,
    help: &str,
    max_characters: usize,
) {
    let label = draw_field_label(ui, label, help);
    ui.add(
        TextEdit::singleline(value)
            .hint_text(hint)
            .char_limit(max_characters)
            .desired_width(f32::INFINITY),
    )
    .on_hover_text(help)
    .labelled_by(label.id);
    ui.end_row();
}
