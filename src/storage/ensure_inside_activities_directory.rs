//! Safety check performed before any destructive operation.

use std::path::{Component, Path};

use crate::error::AppError;

/// Ensures `folder` is a direct child of `activities_directory`.
///
/// This guards renames and deletions against ever touching a path outside
/// the activities folder, whatever the content of the files on disk.
///
/// # Errors
///
/// Returns [`AppError::OutsideActivitiesFolder`] when `folder` is not a
/// direct, normal child of `activities_directory`.
pub fn ensure_inside_activities_directory(
    activities_directory: &Path,
    folder: &Path,
) -> Result<(), AppError> {
    let is_direct_child = folder.parent() == Some(activities_directory);
    let is_normal_name = matches!(folder.components().next_back(), Some(Component::Normal(_)));

    if is_direct_child && is_normal_name {
        Ok(())
    } else {
        Err(AppError::OutsideActivitiesFolder(folder.to_path_buf()))
    }
}
