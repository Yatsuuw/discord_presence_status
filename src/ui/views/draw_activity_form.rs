//! Drawing of the fields of the activity editor.

use eframe::egui::{self, ComboBox, Grid};

use crate::activity::ActivityKind;
use crate::constants::{MAX_IMAGE_REFERENCE_LENGTH, MAX_PRESENCE_TEXT_LENGTH};
use crate::ui::UiAction;
use crate::ui::state::{ActivityDraft, activity_kind_label};
use crate::ui::views::{draw_image_picker, draw_text_field};

/// Longest Discord application ID, in digits.
const MAX_APPLICATION_ID_LENGTH: usize = 20;

/// Help texts of the form fields.
const NAME_HELP: &str = "The name Discord displays, e.g. \"Playing Rust programming\". \
Tip: give your Discord application the same name.";
const APPLICATION_ID_HELP: &str = "Create an application in the Discord Developer Portal, \
then copy its \"Application ID\" here.";
const KIND_HELP: &str = "The verb Discord puts in front of the name.";
const DETAILS_HELP: &str = "First line under the name (your description). Optional.";
const STATE_HELP: &str = "Second line under the description. Optional.";
const IMAGE_HELP: &str = "The key of an image uploaded in the Developer Portal \
(Rich Presence > Art Assets), or a public https:// image URL. Optional.";
const TOOLTIP_HELP: &str = "Text shown when hovering the image on Discord. Optional.";
const LOCAL_IMAGE_HELP: &str = "A copy of the image kept in the activity folder, \
shown here for reference. Discord cannot read files from your computer: \
use the \"Discord image\" field for what Discord displays.";

/// Draws every field of the activity form, editing `draft` in place.
///
/// Returns [`UiAction::PickImage`] when the user wants to choose an image.
pub fn draw_activity_form(ui: &mut egui::Ui, draft: &mut ActivityDraft) -> Option<UiAction> {
    let mut action = None;

    Grid::new("activity_form")
        .num_columns(2)
        .spacing([16.0, 10.0])
        .show(ui, |ui| {
            draw_text_field(
                ui,
                "Name *",
                &mut draft.name,
                "e.g. Rust programming",
                NAME_HELP,
                MAX_PRESENCE_TEXT_LENGTH,
            );
            draw_text_field(
                ui,
                "Application ID *",
                &mut draft.application_id,
                "e.g. 123456789012345678",
                APPLICATION_ID_HELP,
                MAX_APPLICATION_ID_LENGTH,
            );

            let kind_label = ui.label("Activity type").on_hover_text(KIND_HELP);
            ComboBox::from_id_salt("activity_kind")
                .selected_text(activity_kind_label(draft.kind))
                .show_ui(ui, |ui| {
                    for kind in ActivityKind::ALL {
                        ui.selectable_value(&mut draft.kind, kind, activity_kind_label(kind));
                    }
                })
                .response
                .labelled_by(kind_label.id);
            ui.end_row();

            draw_text_field(
                ui,
                "Description",
                &mut draft.details,
                "First line under the name",
                DETAILS_HELP,
                MAX_PRESENCE_TEXT_LENGTH,
            );
            draw_text_field(
                ui,
                "Second line",
                &mut draft.state,
                "Optional",
                STATE_HELP,
                MAX_PRESENCE_TEXT_LENGTH,
            );
            draw_text_field(
                ui,
                "Discord image",
                &mut draft.large_image,
                "Asset key or https:// URL",
                IMAGE_HELP,
                MAX_IMAGE_REFERENCE_LENGTH,
            );
            draw_text_field(
                ui,
                "Image tooltip",
                &mut draft.large_text,
                "Shown when hovering the image",
                TOOLTIP_HELP,
                MAX_PRESENCE_TEXT_LENGTH,
            );

            ui.label("Local image").on_hover_text(LOCAL_IMAGE_HELP);
            action = draw_image_picker(ui, draft);
            ui.end_row();
        });

    action
}
