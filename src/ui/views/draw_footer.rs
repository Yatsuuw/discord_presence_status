//! Drawing of the footer (messages and activities folder).

use std::path::Path;

use eframe::egui::{self, Align, Layout, RichText};

use crate::i18n::{Translations, fill_placeholders};
use crate::ui::UiAction;
use crate::ui::state::{Notice, NoticeKind};
use crate::ui::views::{ERROR_COLOR, SUCCESS_COLOR};

/// Draws the last message and the button opening the activities folder.
pub fn draw_footer(
    ui: &mut egui::Ui,
    notice: Option<&Notice>,
    activities_directory: &Path,
    texts: &Translations,
) -> Option<UiAction> {
    let mut action = None;
    ui.add_space(4.0);
    ui.horizontal(|ui| {
        if let Some(notice) = notice {
            let color = match notice.kind {
                NoticeKind::Success => SUCCESS_COLOR,
                NoticeKind::Error => ERROR_COLOR,
            };
            if ui.small_button(texts.dismiss_button).clicked() {
                action = Some(UiAction::DismissNotice);
            }
            ui.label(RichText::new(&notice.text).color(color));
        }
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            let help = fill_placeholders(
                texts.open_folder_help,
                &[("path", &activities_directory.display().to_string())],
            );
            if ui
                .button(texts.open_folder_button)
                .on_hover_text(help)
                .clicked()
            {
                action = Some(UiAction::OpenActivitiesFolder);
            }
        });
    });
    ui.add_space(4.0);
    action
}
