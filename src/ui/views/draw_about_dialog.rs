//! Drawing of the "About" dialog (version, copyright and license).

use eframe::egui::{self, Id, Modal};

use crate::constants::{APP_DISPLAY_NAME, COPYRIGHT_NOTICE, REPOSITORY_URL};
use crate::i18n::{Translations, fill_placeholders};
use crate::ui::UiAction;

/// Width of the dialog, in logical pixels.
const DIALOG_WIDTH: f32 = 380.0;

/// Draws a modal with the application version, the copyright notice and a
/// summary of the license.
///
/// Returns [`UiAction::CloseAbout`] when the user closes it (button, Escape
/// or click outside).
#[must_use]
pub fn draw_about_dialog(context: &egui::Context, texts: &Translations) -> Option<UiAction> {
    let mut action = None;

    let modal = Modal::new(Id::unique("about_dialog")).show(context, |ui| {
        ui.set_width(DIALOG_WIDTH);
        ui.heading(APP_DISPLAY_NAME);
        ui.label(fill_placeholders(
            texts.about_version,
            &[("version", env!("CARGO_PKG_VERSION"))],
        ));
        ui.add_space(8.0);
        ui.strong(format!("{COPYRIGHT_NOTICE}. {}", texts.about_rights));
        ui.add_space(6.0);
        ui.label(texts.about_license);
        ui.hyperlink_to(texts.about_repository_link, REPOSITORY_URL);
        ui.add_space(6.0);
        ui.small(texts.about_discord_notice);
        ui.add_space(12.0);
        if ui.button(texts.close_button).clicked() {
            action = Some(UiAction::CloseAbout);
        }
    });

    if action.is_none() && modal.should_close() {
        action = Some(UiAction::CloseAbout);
    }
    action
}
