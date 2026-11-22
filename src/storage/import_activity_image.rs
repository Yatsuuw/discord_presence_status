//! Copy of a user-chosen image into an activity folder.

use std::path::Path;

use crate::constants::{ACTIVITY_IMAGE_STEM, MAX_IMAGE_SIZE_BYTES, SUPPORTED_IMAGE_EXTENSIONS};
use crate::error::{AppError, io_error_at};
use crate::storage::remove_activity_images;

/// Number of bytes in a megabyte, used for error messages.
const BYTES_PER_MEGABYTE: u64 = 1024 * 1024;

/// Copies `source` into `folder` as `image.<extension>` and returns that name.
///
/// Any previous local image of the activity is replaced. The copy goes
/// through a temporary file so that a failure never loses the old image.
///
/// # Errors
///
/// * [`AppError::UnsupportedImageFormat`] for an unsupported extension;
/// * [`AppError::ImageTooLarge`] when the file exceeds the size limit;
/// * [`AppError::Io`] when reading or writing fails.
pub fn import_activity_image(folder: &Path, source: &Path) -> Result<String, AppError> {
    let extension = source
        .extension()
        .map(|extension| extension.to_string_lossy().to_lowercase())
        .filter(|extension| SUPPORTED_IMAGE_EXTENSIONS.contains(&extension.as_str()))
        .ok_or_else(|| AppError::UnsupportedImageFormat(SUPPORTED_IMAGE_EXTENSIONS.join(", ")))?;

    let size = std::fs::metadata(source)
        .map_err(io_error_at(source))?
        .len();
    if size > MAX_IMAGE_SIZE_BYTES {
        return Err(AppError::ImageTooLarge {
            max_megabytes: MAX_IMAGE_SIZE_BYTES / BYTES_PER_MEGABYTE,
        });
    }

    let file_name = format!("{ACTIVITY_IMAGE_STEM}.{extension}");
    let target = folder.join(&file_name);
    let temporary = folder.join(format!(".{file_name}.tmp"));

    std::fs::copy(source, &temporary).map_err(io_error_at(source))?;
    remove_activity_images(folder)?;
    std::fs::rename(&temporary, &target).map_err(io_error_at(&target))?;

    Ok(file_name)
}
