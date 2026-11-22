//! Construction of the application state at start-up.

use eframe::CreationContext;

use crate::error::AppError;
use crate::paths::{activities_directory, data_root_directory};
use crate::ui::actions::{show_error, sync_tray_menu};
use crate::ui::build_presence_app;
use crate::ui::state::PresenceApp;
use crate::ui::tray::create_system_tray;

/// Builds the application state for the real installation: locates the data
/// folder, delegates to [`build_presence_app`], then adds the tray icon.
///
/// The application still starts without a tray icon if the system refuses
/// to create one; closing the window then quits, as usual.
///
/// # Errors
///
/// Returns an [`AppError`] when the data folder cannot be located or the
/// presence worker cannot be started.
pub fn create_presence_app(
    creation_context: &CreationContext<'_>,
) -> Result<PresenceApp, AppError> {
    let activities_directory = activities_directory(&data_root_directory()?);
    let mut app = build_presence_app(&creation_context.egui_ctx, activities_directory)?;

    match create_system_tray(&creation_context.egui_ctx) {
        Ok(tray) => {
            app.tray = Some(tray);
            sync_tray_menu(&mut app);
        }
        Err(error) => show_error(&mut app, error),
    }
    Ok(app)
}
