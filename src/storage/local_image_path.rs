//! Resolution of the local image of a stored activity.

use std::path::{Path, PathBuf};

use crate::storage::StoredActivity;

/// Returns the full path of the local image of `stored`, if it exists.
///
/// The file name read from `activity.toml` must be a plain file name: values
/// such as `../secret.png` are ignored so that a hand-edited file can never
/// make the application read outside the activity folder.
#[must_use]
pub fn local_image_path(stored: &StoredActivity) -> Option<PathBuf> {
    let file_name = stored.activity.local_image.as_deref()?;
    let is_plain_file_name = Path::new(file_name).file_name() == Some(file_name.as_ref());

    let path = stored.folder.join(file_name);
    (is_plain_file_name && path.is_file()).then_some(path)
}
