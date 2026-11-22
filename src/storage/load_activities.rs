//! Loading of every activity found in the activities folder.

use std::path::Path;

use crate::constants::ACTIVITY_FILE_NAME;
use crate::error::{AppError, io_error_at};
use crate::storage::{StoredActivity, ensure_directory, read_activity_file};

/// Loads all activities stored in `activities_directory`.
///
/// The folder is created when missing. Sub-folders without an
/// `activity.toml` file are ignored. A broken activity does not prevent the
/// others from loading: each entry of the result is either a loaded activity
/// or the error explaining why that folder could not be read.
///
/// Loaded activities come first, sorted by name (ignoring case), followed by
/// the errors.
///
/// # Errors
///
/// Returns [`AppError::Io`] when the activities folder itself cannot be
/// created or listed.
pub fn load_activities(
    activities_directory: &Path,
) -> Result<Vec<Result<StoredActivity, AppError>>, AppError> {
    ensure_directory(activities_directory)?;

    let entries =
        std::fs::read_dir(activities_directory).map_err(io_error_at(activities_directory))?;

    let mut results: Vec<Result<StoredActivity, AppError>> = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|folder| folder.join(ACTIVITY_FILE_NAME).is_file())
        .map(|folder| {
            read_activity_file(&folder).map(|activity| StoredActivity { folder, activity })
        })
        .collect();

    results.sort_by_cached_key(|result| match result {
        Ok(stored) => (false, stored.activity.name.to_lowercase()),
        Err(_) => (true, String::new()),
    });
    Ok(results)
}
