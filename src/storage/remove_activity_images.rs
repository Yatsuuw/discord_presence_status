//! Removal of the local images of an activity.

use std::path::Path;

use crate::constants::{ACTIVITY_IMAGE_STEM, SUPPORTED_IMAGE_EXTENSIONS};
use crate::error::{AppError, io_error_at};

/// Deletes every `image.<extension>` file found in `folder`.
///
/// Only files the application itself creates are touched; anything else the
/// user put in the folder is left alone.
///
/// # Errors
///
/// Returns [`AppError::Io`] when an existing image cannot be deleted.
pub fn remove_activity_images(folder: &Path) -> Result<(), AppError> {
    for extension in SUPPORTED_IMAGE_EXTENSIONS {
        let path = folder.join(format!("{ACTIVITY_IMAGE_STEM}.{extension}"));
        if path.is_file() {
            std::fs::remove_file(&path).map_err(io_error_at(&path))?;
        }
    }
    Ok(())
}
