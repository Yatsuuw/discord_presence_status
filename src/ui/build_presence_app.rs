//! Construction of the application state for a given data folder.

use std::path::Path;
use std::sync::{Arc, Mutex};

use eframe::egui;

use crate::error::AppError;
use crate::paths::activities_directory;
use crate::presence::{PresenceStatus, spawn_presence_worker};
use crate::settings::{Settings, load_settings, settings_file_path};
use crate::ui::actions::{reload_activities, report_error};
use crate::ui::repaint_window;
use crate::ui::state::{PresenceApp, RepaintTarget};

/// Builds the application state for the data stored in `data_root`:
/// loads the preferences (`settings.toml`), starts the presence worker and
/// loads the activities of `user_activity/`.
///
/// `context` is the window the worker wakes up, if one is already open;
/// [`run_app`](crate::ui::run_app) passes `None` and sets it for each window
/// it creates. Taking the folder as a parameter lets tests run the real
/// interface on a temporary folder. Unreadable preferences are reported and
/// replaced by the defaults instead of preventing the start. The tray icon
/// is not created here, so tests never touch the notification area.
///
/// # Errors
///
/// Returns [`AppError::WorkerSpawn`] when the presence worker cannot start.
pub fn build_presence_app(
    context: Option<&egui::Context>,
    data_root: &Path,
) -> Result<PresenceApp, AppError> {
    if let Some(context) = context {
        egui_extras::install_image_loaders(context);
    }
    let repaint_target: RepaintTarget = Arc::new(Mutex::new(context.cloned()));

    let worker_target = Arc::clone(&repaint_target);
    let presence = spawn_presence_worker(Arc::new(move || repaint_window(&worker_target)))?;

    let settings_path = settings_file_path(data_root);
    let settings = load_settings(&settings_path);

    let mut app = PresenceApp {
        activities_directory: activities_directory(data_root),
        settings_path,
        language: settings.as_ref().map_or_else(
            |_| Settings::default().language,
            |settings| settings.language,
        ),
        activities: Vec::new(),
        load_errors: Vec::new(),
        draft: None,
        notice: None,
        presence,
        status: PresenceStatus::Idle,
        active: None,
        dialog: None,
        tray: None,
        window_exit: None,
        repaint_target,
    };
    reload_activities(&mut app);
    if let Err(error) = settings {
        report_error(&mut app, &error);
    }
    Ok(app)
}
