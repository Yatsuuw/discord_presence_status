//! Construction of the application state at start-up.

use eframe::CreationContext;

use crate::error::AppError;
use crate::paths::{activities_directory, data_root_directory};
use crate::ui::build_presence_app;
use crate::ui::state::PresenceApp;

/// Builds the application state for the real installation: locates the data
/// folder, then delegates to [`build_presence_app`].
///
/// # Errors
///
/// Returns an [`AppError`] when the data folder cannot be located or the
/// presence worker cannot be started.
pub fn create_presence_app(
    creation_context: &CreationContext<'_>,
) -> Result<PresenceApp, AppError> {
    let activities_directory = activities_directory(&data_root_directory()?);
    build_presence_app(&creation_context.egui_ctx, activities_directory)
}
