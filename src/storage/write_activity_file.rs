//! Writing of an `activity.toml` file.

use std::path::Path;

use crate::activity::Activity;
use crate::constants::ACTIVITY_FILE_NAME;
use crate::error::{AppError, io_error_at};

/// Comment written at the top of every activity file.
const FILE_HEADER: &str = "# Activity managed by Discord Presence Status.\n\
# You may edit this file by hand while the application is closed.\n\n";

/// Writes `activity` to `folder/activity.toml`.
///
/// The content is first written to a temporary file which then replaces the
/// previous one, so a crash never leaves a half-written activity behind.
///
/// # Errors
///
/// Returns [`AppError::ActivitySerialize`] or [`AppError::Io`] on failure.
pub fn write_activity_file(folder: &Path, activity: &Activity) -> Result<(), AppError> {
    let content = format!("{FILE_HEADER}{}", toml::to_string_pretty(activity)?);

    let path = folder.join(ACTIVITY_FILE_NAME);
    let temporary_path = folder.join(format!("{ACTIVITY_FILE_NAME}.tmp"));

    std::fs::write(&temporary_path, content).map_err(io_error_at(&temporary_path))?;
    std::fs::rename(&temporary_path, &path).map_err(io_error_at(&path))
}
