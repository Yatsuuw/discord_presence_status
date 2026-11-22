//! Location of the settings file.

use std::path::{Path, PathBuf};

use crate::constants::SETTINGS_FILE_NAME;

/// Returns the path of `settings.toml` inside the given data root.
#[must_use]
pub fn settings_file_path(data_root: &Path) -> PathBuf {
    data_root.join(SETTINGS_FILE_NAME)
}
