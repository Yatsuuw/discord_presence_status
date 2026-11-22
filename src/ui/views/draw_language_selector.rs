//! Drawing of the language selector.

use eframe::egui::{self, ComboBox};

use crate::i18n::{Language, Translations, language_name};
use crate::ui::UiAction;

/// Draws a drop-down list of the supported languages.
///
/// Returns [`UiAction::ChangeLanguage`] when the user picks another one.
pub fn draw_language_selector(
    ui: &mut egui::Ui,
    current: Language,
    texts: &Translations,
) -> Option<UiAction> {
    let mut selected = current;
    ComboBox::from_id_salt("language")
        .selected_text(language_name(current))
        .show_ui(ui, |ui| {
            for language in Language::ALL {
                ui.selectable_value(&mut selected, language, language_name(language));
            }
        })
        .response
        .on_hover_text(texts.language_help)
        .widget_info(|| {
            egui::WidgetInfo::labeled(egui::Role::ComboBox, true, texts.language_label)
        });

    (selected != current).then_some(UiAction::ChangeLanguage(selected))
}
