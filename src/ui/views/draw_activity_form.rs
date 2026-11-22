//! Drawing of the fields of the activity editor.

use eframe::egui::{self, ComboBox, Grid};

use crate::activity::ActivityKind;
use crate::constants::{MAX_IMAGE_REFERENCE_LENGTH, MAX_PRESENCE_TEXT_LENGTH};
use crate::i18n::{Translations, activity_kind_label};
use crate::ui::UiAction;
use crate::ui::state::ActivityDraft;
use crate::ui::views::{draw_field_label, draw_image_picker, draw_text_field};

/// Longest Discord application ID, in digits.
const MAX_APPLICATION_ID_LENGTH: usize = 20;

/// Draws every field of the activity form, editing `draft` in place.
///
/// Each label is followed by a "(?)" icon explaining what to enter.
/// Returns [`UiAction::PickImage`] when the user wants to choose an image.
pub fn draw_activity_form(
    ui: &mut egui::Ui,
    draft: &mut ActivityDraft,
    texts: &Translations,
) -> Option<UiAction> {
    let mut action = None;

    Grid::new("activity_form")
        .num_columns(2)
        .spacing([16.0, 10.0])
        .show(ui, |ui| {
            draw_text_field(
                ui,
                texts.field_name,
                &mut draft.name,
                texts.hint_name,
                texts.help_name,
                MAX_PRESENCE_TEXT_LENGTH,
            );
            draw_text_field(
                ui,
                texts.field_application_id,
                &mut draft.application_id,
                texts.hint_application_id,
                texts.help_application_id,
                MAX_APPLICATION_ID_LENGTH,
            );

            let kind_label = draw_field_label(ui, texts.field_kind, texts.help_kind);
            ComboBox::from_id_salt("activity_kind")
                .selected_text(activity_kind_label(draft.kind, texts))
                .show_ui(ui, |ui| {
                    for kind in ActivityKind::ALL {
                        ui.selectable_value(
                            &mut draft.kind,
                            kind,
                            activity_kind_label(kind, texts),
                        );
                    }
                })
                .response
                .on_hover_text(texts.help_kind)
                .labelled_by(kind_label.id);
            ui.end_row();

            draw_text_field(
                ui,
                texts.field_details,
                &mut draft.details,
                texts.hint_details,
                texts.help_details,
                MAX_PRESENCE_TEXT_LENGTH,
            );
            draw_text_field(
                ui,
                texts.field_state,
                &mut draft.state,
                texts.hint_state,
                texts.help_state,
                MAX_PRESENCE_TEXT_LENGTH,
            );
            draw_text_field(
                ui,
                texts.field_large_image,
                &mut draft.large_image,
                texts.hint_large_image,
                texts.help_large_image,
                MAX_IMAGE_REFERENCE_LENGTH,
            );
            draw_text_field(
                ui,
                texts.field_large_text,
                &mut draft.large_text,
                texts.hint_large_text,
                texts.help_large_text,
                MAX_PRESENCE_TEXT_LENGTH,
            );

            draw_field_label(ui, texts.field_local_image, texts.help_local_image);
            action = draw_image_picker(ui, draft, texts);
            ui.end_row();
        });

    action
}
