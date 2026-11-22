//! Drawing of the local image preview and its buttons.

use eframe::egui::{self, Image, Vec2};

use crate::ui::UiAction;
use crate::ui::state::ActivityDraft;

/// Largest size of the preview, in logical pixels.
const PREVIEW_SIZE: f32 = 72.0;

/// Corner rounding of the preview, in logical pixels.
const PREVIEW_ROUNDING: f32 = 8.0;

/// Draws the local image preview with "Choose" and "Remove" buttons.
///
/// Removing only clears the form; the file is deleted when the activity is
/// saved. Returns [`UiAction::PickImage`] when the user wants a new image.
pub fn draw_image_picker(ui: &mut egui::Ui, draft: &mut ActivityDraft) -> Option<UiAction> {
    let mut action = None;
    ui.horizontal(|ui| {
        match &draft.preview_image {
            Some(path) => {
                ui.add(
                    Image::new(format!("file://{}", path.display()))
                        .max_size(Vec2::splat(PREVIEW_SIZE))
                        .corner_radius(PREVIEW_ROUNDING),
                );
            }
            None => {
                ui.weak("No image");
            }
        }
        ui.vertical(|ui| {
            if ui.button("Choose image...").clicked() {
                action = Some(UiAction::PickImage);
            }
            if draft.preview_image.is_some() && ui.button("Remove image").clicked() {
                draft.local_image = None;
                draft.new_image = None;
                draft.preview_image = None;
            }
        });
    });
    action
}
