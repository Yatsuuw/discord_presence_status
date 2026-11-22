//! Creation of a folder when it is missing.

use std::path::Path;

use crate::error::{AppError, io_error_at};

/// Creates `folder` and all its missing parents. Does nothing if it exists.
///
/// # Errors
///
/// Returns [`AppError::Io`] when the folder cannot be created.
pub fn ensure_directory(folder: &Path) -> Result<(), AppError> {
    std::fs::create_dir_all(folder).map_err(io_error_at(folder))
}
