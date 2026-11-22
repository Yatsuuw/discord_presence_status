//! Routing of user actions to their implementation.

use eframe::egui;

use crate::ui::UiAction;
use crate::ui::actions::{
    create_new_activity, delete_selected_activity, open_activities_folder, pick_image,
    reload_activities, save_draft, select_activity, start_selected_activity, stop_presence,
};
use crate::ui::state::PresenceApp;

/// Applies `action` to the application state.
pub fn apply_action(app: &mut PresenceApp, action: UiAction, context: &egui::Context) {
    match action {
        UiAction::NewActivity => create_new_activity(app),
        UiAction::Select(folder) => select_activity(app, &folder),
        UiAction::Save => {
            let _ = save_draft(app, context);
        }
        UiAction::StartPresence => start_selected_activity(app, context),
        UiAction::StopPresence => stop_presence(app),
        UiAction::PickImage => pick_image(app),
        UiAction::RequestDelete => app.delete_requested = true,
        UiAction::ConfirmDelete => delete_selected_activity(app),
        UiAction::CancelDelete => app.delete_requested = false,
        UiAction::OpenActivitiesFolder => open_activities_folder(app),
        UiAction::Reload => reload_activities(app),
        UiAction::DismissNotice => app.notice = None,
    }
}
