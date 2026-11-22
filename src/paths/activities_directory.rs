//! Location of the `user_activity/` folder.

use std::path::{Path, PathBuf};

use crate::constants::ACTIVITIES_FOLDER_NAME;

/// Returns the `user_activity/` folder inside the given data root.
///
/// ```
/// use std::path::Path;
/// use discord_presence_status::paths::activities_directory;
///
/// let folder = activities_directory(Path::new("data"));
/// assert_eq!(folder, Path::new("data").join("user_activity"));
/// ```
#[must_use]
pub fn activities_directory(data_root: &Path) -> PathBuf {
    data_root.join(ACTIVITIES_FOLDER_NAME)
}
