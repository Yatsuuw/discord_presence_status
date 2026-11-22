//! Construction of the application state for a given activities folder.

use std::path::PathBuf;
use std::sync::Arc;

use eframe::egui;

use crate::error::AppError;
use crate::presence::{PresenceStatus, spawn_presence_worker};
use crate::ui::actions::reload_activities;
use crate::ui::state::PresenceApp;

/// Builds the application state: installs the image loaders, starts the
/// presence worker and loads the activities of `activities_directory`.
///
/// Taking the folder as a parameter lets tests run the real interface on a
/// temporary folder.
///
/// # Errors
///
/// Returns [`AppError::WorkerSpawn`] when the presence worker cannot start.
pub fn build_presence_app(
    context: &egui::Context,
    activities_directory: PathBuf,
) -> Result<PresenceApp, AppError> {
    egui_extras::install_image_loaders(context);

    let repaint_context = context.clone();
    let presence = spawn_presence_worker(Arc::new(move || repaint_context.request_repaint()))?;

    let mut app = PresenceApp {
        activities_directory,
        activities: Vec::new(),
        load_errors: Vec::new(),
        draft: None,
        notice: None,
        presence,
        status: PresenceStatus::Idle,
        active: None,
        delete_requested: false,
    };
    reload_activities(&mut app);
    Ok(app)
}
