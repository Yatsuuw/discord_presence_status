//! Drawing of the activity editor (central panel).

use eframe::egui::{self, RichText, ScrollArea};

use crate::i18n::Translations;
use crate::ui::UiAction;
use crate::ui::state::ActivityDraft;
use crate::ui::views::{SUCCESS_COLOR, draw_activity_form, draw_editor_buttons};

/// Address of the Discord Developer Portal.
const DEVELOPER_PORTAL_URL: &str = "https://discord.com/developers/applications";

/// Draws the editor (title, form and buttons) and returns the action the user triggered.
///
/// `is_active` tells whether this activity is the one displayed on Discord.
pub fn draw_activity_editor(
    ui: &mut egui::Ui,
    draft: &mut ActivityDraft,
    is_active: bool,
    texts: &Translations,
) -> Option<UiAction> {
    let mut action = None;

    ScrollArea::vertical().show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.heading(if draft.folder.is_some() {
                texts.editor_title_edit
            } else {
                texts.editor_title_new
            });
            if is_active {
                ui.label(RichText::new(texts.live_badge).color(SUCCESS_COLOR));
            }
        });
        ui.add_space(10.0);

        action = draw_activity_form(ui, draft, texts);

        ui.add_space(12.0);
        ui.separator();
        ui.add_space(4.0);

        action = action.take().or(draw_editor_buttons(ui, is_active, texts));

        ui.add_space(12.0);
        ui.weak(texts.required_hint);
        ui.hyperlink_to(texts.portal_link, DEVELOPER_PORTAL_URL);
    });

    action
}
