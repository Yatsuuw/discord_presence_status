//! Construction of the application state for a given activities folder.

use std::path::Path;
use std::sync::Arc;

use eframe::egui;

use crate::error::AppError;
use crate::paths::activities_directory;
use crate::presence::{PresenceStatus, spawn_presence_worker};
use crate::settings::{Settings, load_settings, settings_file_path};
use crate::ui::actions::{reload_activities, report_error};
use crate::ui::state::PresenceApp;

/// Builds the application state for the data stored in `data_root`:
/// installs the image loaders, loads the preferences (`settings.toml`),
/// starts the presence worker and loads the activities of `user_activity/`.
///
/// Taking the folder as a parameter lets tests run the real interface on a
/// temporary folder. Unreadable preferences are reported and replaced by the
/// defaults instead of preventing the start. The tray icon is not created here (see
/// [`create_presence_app`](crate::ui::create_presence_app)), so tests never
/// touch the notification area.
///
/// # Errors
///
/// Returns [`AppError::WorkerSpawn`] when the presence worker cannot start.
pub fn build_presence_app(
    context: &egui::Context,
    data_root: &Path,
) -> Result<PresenceApp, AppError> {
    egui_extras::install_image_loaders(context);

    let repaint_context = context.clone();
    let presence = spawn_presence_worker(Arc::new(move || repaint_context.request_repaint()))?;

    let settings_path = settings_file_path(data_root);
    let settings = load_settings(&settings_path);

    let mut app = PresenceApp {
        activities_directory: activities_directory(data_root),
        settings_path,
        language: settings
            .as_ref()
            .map_or_else(|_| Settings::default().language, |s| s.language),
        activities: Vec::new(),
        load_errors: Vec::new(),
        draft: None,
        notice: None,
        presence,
        status: PresenceStatus::Idle,
        active: None,
        dialog: None,
        tray: None,
        window_hidden: false,
        quit_requested: false,
    };
    reload_activities(&mut app);
    if let Err(error) = settings {
        report_error(&mut app, &error);
    }
    Ok(app)
}
