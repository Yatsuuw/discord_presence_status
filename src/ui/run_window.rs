//! Opening of one main window.

use crate::constants::APP_DISPLAY_NAME;
use crate::error::AppError;
use crate::ui::build_native_options;
use crate::ui::state::PresenceApp;
use crate::ui::window_session::WindowSession;

/// Opens the main window on `app` and returns once it is closed.
///
/// Each window gets a fresh egui context, published in `repaint_target` so
/// the presence worker and the tray icon can wake it up, and withdrawn when
/// the window closes. Its images are loaded again from disk.
///
/// # Errors
///
/// Returns [`AppError::Interface`] when the window cannot be created.
pub fn run_window(app: &mut PresenceApp) -> Result<(), AppError> {
    app.window_exit = None;
    let repaint_target = std::sync::Arc::clone(&app.repaint_target);

    let result = eframe::run_native(
        APP_DISPLAY_NAME,
        build_native_options(),
        Box::new(|creation_context| {
            egui_extras::install_image_loaders(&creation_context.egui_ctx);
            if let Ok(mut target) = repaint_target.lock() {
                *target = Some(creation_context.egui_ctx.clone());
            }
            Ok(Box::new(WindowSession { app }))
        }),
    );

    if let Ok(mut target) = repaint_target.lock() {
        *target = None;
    }
    result.map_err(|error| AppError::Interface(error.to_string()))
}
