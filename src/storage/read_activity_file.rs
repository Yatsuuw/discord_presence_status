//! Reading of an `activity.toml` file.

use std::path::Path;

use crate::activity::Activity;
use crate::constants::ACTIVITY_FILE_NAME;
use crate::error::{AppError, io_error_at};

/// Reads the activity stored in `folder/activity.toml`.
///
/// # Errors
///
/// Returns [`AppError::Io`] when the file cannot be read and
/// [`AppError::ActivityParse`] when its content is not a valid activity.
pub fn read_activity_file(folder: &Path) -> Result<Activity, AppError> {
    let path = folder.join(ACTIVITY_FILE_NAME);
    let content = std::fs::read_to_string(&path).map_err(io_error_at(&path))?;
    toml::from_str(&content).map_err(|source| AppError::ActivityParse { path, source })
}
