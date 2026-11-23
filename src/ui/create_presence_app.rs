//! Construction of the application state at start-up.

use crate::error::AppError;
use crate::paths::data_root_directory;
use crate::ui::actions::{report_error, sync_tray_menu};
use crate::ui::build_presence_app;
use crate::ui::state::PresenceApp;
use crate::ui::tray::create_system_tray;

/// Builds the application state for the real installation: locates the data
/// folder, delegates to [`build_presence_app`], then adds the tray icon.
///
/// No window exists yet: [`run_app`](crate::ui::run_app) opens one right
/// after. The application still starts without a tray icon if the system
/// refuses to create one; closing the window then quits, as usual.
///
/// # Errors
///
/// Returns an [`AppError`] when the data folder cannot be located or the
/// presence worker cannot be started.
pub fn create_presence_app() -> Result<PresenceApp, AppError> {
    let mut app = build_presence_app(None, &data_root_directory()?)?;

    match create_system_tray(&app.repaint_target) {
        Ok(tray) => {
            app.tray = Some(tray);
            sync_tray_menu(&mut app);
        }
        Err(error) => report_error(&mut app, &error),
    }
    Ok(app)
}
