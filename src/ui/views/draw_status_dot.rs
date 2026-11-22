//! Drawing of a small colored status dot.

use eframe::egui::{self, Color32, Sense, vec2};

/// Diameter of the dot, in logical pixels.
const DOT_DIAMETER: f32 = 10.0;

/// Draws a filled circle of `color`, aligned with the surrounding text.
pub fn draw_status_dot(ui: &mut egui::Ui, color: Color32) {
    let (rect, _) = ui.allocate_exact_size(vec2(DOT_DIAMETER, DOT_DIAMETER), Sense::hover());
    ui.painter()
        .circle_filled(rect.center(), DOT_DIAMETER / 2.0, color);
}
