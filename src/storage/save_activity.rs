//! Saving of an activity, including its folder and local image.

use std::path::Path;

use crate::activity::{Activity, validate_activity};
use crate::error::AppError;
use crate::storage::{
    StoredActivity, import_activity_image, prepare_activity_folder, remove_activity_images,
    write_activity_file,
};

/// Validates and saves `activity` in the activities folder.
///
/// * `previous_folder`: folder of the activity before this save, `None` for a
///   new activity. When the name changed, that folder is renamed.
/// * `new_image`: image chosen by the user since the last save, copied into
///   the activity folder and referenced in `activity.toml`. When there is no
///   new image and `activity.local_image` is `None`, any previous local image
///   is deleted (the user removed it).
///
/// # Errors
///
/// Returns [`AppError::Validation`] when the activity is invalid, or any
/// storage error raised while preparing the folder, copying the image or
/// writing the file.
pub fn save_activity(
    activities_directory: &Path,
    mut activity: Activity,
    previous_folder: Option<&Path>,
    new_image: Option<&Path>,
) -> Result<StoredActivity, AppError> {
    validate_activity(&activity)?;

    let folder = prepare_activity_folder(activities_directory, &activity.name, previous_folder)?;

    if let Some(image) = new_image {
        activity.local_image = Some(import_activity_image(&folder, image)?);
    } else if activity.local_image.is_none() {
        remove_activity_images(&folder)?;
    }

    write_activity_file(&folder, &activity)?;
    Ok(StoredActivity { folder, activity })
}
