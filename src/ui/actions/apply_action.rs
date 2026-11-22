//! Routing of user actions to their implementation.

use eframe::egui;

use crate::ui::UiAction;
use crate::ui::actions::{
    change_language, create_new_activity, delete_selected_activity, hide_to_tray,
    open_activities_folder, pick_image, quit_application, reload_activities, save_draft,
    select_activity, show_window, start_activity, start_selected_activity, stop_presence,
};
use crate::ui::state::{Dialog, PresenceApp};

/// Applies `action` to the application state.
pub fn apply_action(app: &mut PresenceApp, action: UiAction, context: &egui::Context) {
    match action {
        UiAction::NewActivity => create_new_activity(app),
        UiAction::Select(folder) => select_activity(app, &folder),
        UiAction::Save => {
            let _ = save_draft(app, context);
        }
        UiAction::StartPresence => start_selected_activity(app, context),
        UiAction::StartActivity(folder) => start_activity(app, &folder),
        UiAction::StopPresence => stop_presence(app),
        UiAction::PickImage => pick_image(app),
        UiAction::RequestDelete => app.dialog = Some(Dialog::DeleteConfirmation),
        UiAction::ConfirmDelete => delete_selected_activity(app),
        UiAction::CancelDelete | UiAction::CloseAbout => app.dialog = None,
        UiAction::OpenActivitiesFolder => open_activities_folder(app),
        UiAction::Reload => reload_activities(app),
        UiAction::DismissNotice => app.notice = None,
        UiAction::ShowAbout => app.dialog = Some(Dialog::About),
        UiAction::ShowWindow => show_window(app, context),
        UiAction::HideToTray => hide_to_tray(app, context),
        UiAction::Quit => quit_application(app, context),
        UiAction::ChangeLanguage(language) => change_language(app, language),
    }
}
