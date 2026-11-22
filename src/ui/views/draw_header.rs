//! Drawing of the header (title, language and presence status).

use eframe::egui::{self, Align, Layout, RichText};

use crate::constants::APP_DISPLAY_NAME;
use crate::i18n::{Language, Translations, describe_presence_error};
use crate::presence::PresenceStatus;
use crate::ui::UiAction;
use crate::ui::views::{describe_status, draw_language_selector, draw_status_dot};

/// Vertical padding around the header content, in logical pixels.
const PADDING: f32 = 6.0;

/// Draws the application title, the language selector and the current
/// presence status. Returns the action triggered by the language selector.
pub fn draw_header(
    ui: &mut egui::Ui,
    status: &PresenceStatus,
    language: Language,
    texts: &Translations,
) -> Option<UiAction> {
    let display = describe_status(status, texts);
    let mut action = None;

    ui.add_space(PADDING);
    ui.horizontal(|ui| {
        ui.heading(APP_DISPLAY_NAME);
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            action = draw_language_selector(ui, language, texts);
            ui.separator();
            let label = ui.label(RichText::new(&display.text).color(display.color));
            if let Some(detail) = &display.detail {
                label.on_hover_text(detail);
            }
            draw_status_dot(ui, display.color);
        });
    });
    if let PresenceStatus::Waiting { error, .. } | PresenceStatus::Rejected { error, .. } = status {
        ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
            ui.small(describe_presence_error(error, texts));
        });
    }
    ui.add_space(PADDING);
    action
}
