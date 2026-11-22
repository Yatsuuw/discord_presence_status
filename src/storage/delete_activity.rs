//! Deletion of an activity folder.

use std::path::Path;

use crate::error::{AppError, io_error_at};
use crate::storage::ensure_inside_activities_directory;

/// Permanently deletes the activity stored in `folder`, with all its files.
///
/// # Errors
///
/// Returns [`AppError::OutsideActivitiesFolder`] when `folder` is not a
/// direct child of `activities_directory`, or [`AppError::Io`] when the
/// deletion fails.
pub fn delete_activity(activities_directory: &Path, folder: &Path) -> Result<(), AppError> {
    ensure_inside_activities_directory(activities_directory, folder)?;
    std::fs::remove_dir_all(folder).map_err(io_error_at(folder))
}
