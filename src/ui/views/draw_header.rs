//! Drawing of the header (title and presence status).

use eframe::egui::{self, Align, Layout, RichText};

use crate::constants::APP_DISPLAY_NAME;
use crate::presence::PresenceStatus;
use crate::ui::views::{describe_status, draw_status_dot};

/// Vertical padding around the header content, in logical pixels.
const PADDING: f32 = 6.0;

/// Draws the application title and the current presence status.
pub fn draw_header(ui: &mut egui::Ui, status: &PresenceStatus) {
    let display = describe_status(status);

    ui.add_space(PADDING);
    ui.horizontal(|ui| {
        ui.heading(APP_DISPLAY_NAME);
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            let label = ui.label(RichText::new(&display.text).color(display.color));
            if let Some(detail) = &display.detail {
                label.on_hover_text(detail);
            }
            draw_status_dot(ui, display.color);
        });
    });
    if let PresenceStatus::Waiting { reason, .. } | PresenceStatus::Rejected { reason, .. } = status
    {
        ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
            ui.small(reason);
        });
    }
    ui.add_space(PADDING);
}
