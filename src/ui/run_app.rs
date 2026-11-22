//! Start of the desktop application.

use crate::constants::APP_DISPLAY_NAME;
use crate::error::AppError;
use crate::ui::{build_native_options, create_presence_app};

/// Opens the main window and runs the application until it is closed.
///
/// # Errors
///
/// Returns [`AppError::Interface`] when the window cannot be created, or the
/// error raised while preparing the application state.
pub fn run_app() -> Result<(), AppError> {
    eframe::run_native(
        APP_DISPLAY_NAME,
        build_native_options(),
        Box::new(|creation_context| Ok(Box::new(create_presence_app(creation_context)?))),
    )
    .map_err(|error| AppError::Interface(error.to_string()))
}
