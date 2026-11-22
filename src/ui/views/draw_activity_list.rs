//! Drawing of the activity list (left panel).

use std::path::Path;

use eframe::egui::{self, Align, Button, Layout, RichText, ScrollArea};

use crate::storage::StoredActivity;
use crate::ui::UiAction;
use crate::ui::views::{ERROR_COLOR, SUCCESS_COLOR};

/// Height of the "New activity" button, in logical pixels.
const NEW_BUTTON_HEIGHT: f32 = 28.0;

/// Draws the list of activities and returns the action the user triggered.
///
/// * `selected`: folder of the activity open in the editor;
/// * `active`: folder of the activity displayed on Discord.
pub fn draw_activity_list(
    ui: &mut egui::Ui,
    activities: &[StoredActivity],
    load_errors: &[String],
    selected: Option<&Path>,
    active: Option<&Path>,
) -> Option<UiAction> {
    let mut action = None;

    ui.add_space(8.0);
    ui.horizontal(|ui| {
        ui.strong("Activities");
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if ui
                .small_button("Reload")
                .on_hover_text("Read the activities folder again")
                .clicked()
            {
                action = Some(UiAction::Reload);
            }
        });
    });
    ui.add_space(4.0);

    let new_button = Button::new("+ New activity");
    if ui
        .add_sized([ui.available_width(), NEW_BUTTON_HEIGHT], new_button)
        .clicked()
    {
        action = Some(UiAction::NewActivity);
    }
    ui.separator();

    ScrollArea::vertical().show(ui, |ui| {
        ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
            if activities.is_empty() {
                ui.weak("No activity yet. Create your first one!");
            }
            for stored in activities {
                let is_selected = selected == Some(stored.folder.as_path());
                let is_active = active == Some(stored.folder.as_path());
                let text = if is_active {
                    RichText::new(format!("{}  (live)", stored.activity.name)).color(SUCCESS_COLOR)
                } else {
                    RichText::new(&stored.activity.name)
                };
                if ui.selectable_label(is_selected, text).clicked() {
                    action = Some(UiAction::Select(stored.folder.clone()));
                }
            }
            for error in load_errors {
                ui.add_space(4.0);
                ui.label(RichText::new(error).small().color(ERROR_COLOR));
            }
        });
    });

    action
}
