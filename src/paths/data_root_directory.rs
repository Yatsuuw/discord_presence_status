//! Selection of the folder holding all user data.

use std::path::PathBuf;

use crate::error::AppError;
use crate::paths::{executable_directory, is_machine_wide_install, user_data_directory};

/// Returns the folder under which `user_activity/` is stored.
///
/// * per-user installation (and development builds): the executable folder;
/// * machine-wide installation: the per-user data folder, because standard
///   users cannot write into `Program Files`.
///
/// # Errors
///
/// Returns an [`AppError`] when the executable or the per-user data folder
/// cannot be located.
pub fn data_root_directory() -> Result<PathBuf, AppError> {
    let executable_directory = executable_directory()?;
    if is_machine_wide_install(&executable_directory) {
        user_data_directory()
    } else {
        Ok(executable_directory)
    }
}
