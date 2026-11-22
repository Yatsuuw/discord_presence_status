//! Drawing of the small "(?)" help icon placed next to a label.

use eframe::egui::{self, Align2, FontId, Response, Role, Sense, Stroke, Vec2, WidgetInfo};

/// Diameter of the icon, in logical pixels.
const MARKER_SIZE: f32 = 15.0;

/// Size of the question mark, in points.
const MARK_FONT_SIZE: f32 = 10.0;

/// Draws a circled question mark that shows `help` when hovered.
///
/// Screen readers announce the help text itself.
pub fn draw_help_marker(ui: &mut egui::Ui, help: &str) -> Response {
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(MARKER_SIZE), Sense::hover());
    response.widget_info(|| WidgetInfo::labeled(Role::Label, true, help));

    let color = if response.hovered() {
        ui.visuals().strong_text_color()
    } else {
        ui.visuals().weak_text_color()
    };
    let painter = ui.painter();
    painter.circle_stroke(
        rect.center(),
        MARKER_SIZE / 2.0 - 1.0,
        Stroke::new(1.0, color),
    );
    painter.text(
        rect.center(),
        Align2::CENTER_CENTER,
        "?",
        FontId::proportional(MARK_FONT_SIZE),
        color,
    );
    response.on_hover_text(help)
}
