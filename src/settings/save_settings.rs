//! Writing of the settings file.

use std::path::Path;

use crate::error::{AppError, io_error_at};
use crate::settings::Settings;
use crate::storage::ensure_directory;

/// Comment written at the top of the settings file.
const FILE_HEADER: &str = "# Preferences of Discord Presence Status.\n\n";

/// Saves `settings` to `path`, creating its folder if needed.
///
/// The content goes through a temporary file, so a crash never leaves a
/// half-written file behind.
///
/// # Errors
///
/// Returns [`AppError::SettingsSerialize`] or [`AppError::Io`] on failure.
pub fn save_settings(path: &Path, settings: &Settings) -> Result<(), AppError> {
    let content = toml::to_string_pretty(settings).map_err(AppError::SettingsSerialize)?;

    if let Some(folder) = path.parent() {
        ensure_directory(folder)?;
    }
    let temporary_path = path.with_extension("toml.tmp");
    std::fs::write(&temporary_path, format!("{FILE_HEADER}{content}"))
        .map_err(io_error_at(&temporary_path))?;
    std::fs::rename(&temporary_path, path).map_err(io_error_at(path))
}
