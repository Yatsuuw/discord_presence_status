//! Creation or renaming of the folder of an activity.

use std::path::{Path, PathBuf};

use crate::activity::sanitize_folder_name;
use crate::error::{AppError, io_error_at};
use crate::storage::{ensure_directory, ensure_inside_activities_directory, is_same_folder};

/// Returns the folder an activity named `name` must be saved in.
///
/// * new activity (`previous_folder` is `None` or no longer exists): the
///   folder is created;
/// * existing activity whose name did not change: the folder is reused;
/// * renamed activity: the previous folder is renamed, keeping its image.
///
/// # Errors
///
/// Returns [`AppError::ActivityAlreadyExists`] when another activity already
/// uses the target folder, or [`AppError::Io`] when the file system refuses
/// the operation.
pub fn prepare_activity_folder(
    activities_directory: &Path,
    name: &str,
    previous_folder: Option<&Path>,
) -> Result<PathBuf, AppError> {
    ensure_directory(activities_directory)?;
    let target = activities_directory.join(sanitize_folder_name(name.trim()));
    let previous_folder = previous_folder.filter(|folder| folder.is_dir());

    match previous_folder {
        Some(previous) if is_same_folder(previous, &target) => {
            if previous != target {
                // Only the letter case changed: rename in place.
                ensure_inside_activities_directory(activities_directory, previous)?;
                std::fs::rename(previous, &target).map_err(io_error_at(previous))?;
            }
        }
        Some(_) | None if target.exists() => {
            return Err(AppError::ActivityAlreadyExists(name.trim().to_owned()));
        }
        Some(previous) => {
            ensure_inside_activities_directory(activities_directory, previous)?;
            std::fs::rename(previous, &target).map_err(io_error_at(previous))?;
        }
        None => ensure_directory(&target)?,
    }
    Ok(target)
}
