//! Location of the per-user application data folder.

use std::path::PathBuf;

use crate::constants::APP_DISPLAY_NAME;
use crate::error::AppError;

/// Returns the per-user data folder of the application.
///
/// * Windows: `%APPDATA%\Discord Presence Status` (roaming profile);
/// * other systems: `$XDG_DATA_HOME/Discord Presence Status`, falling back to
///   `$HOME/.local/share/Discord Presence Status`.
///
/// The folder is not created by this function.
///
/// # Errors
///
/// Returns [`AppError::MissingAppDataFolder`] when none of the expected
/// environment variables is set.
pub fn user_data_directory() -> Result<PathBuf, AppError> {
    let base = if cfg!(windows) {
        std::env::var_os("APPDATA").map(PathBuf::from)
    } else {
        std::env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .or_else(|| {
                std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/share"))
            })
    };

    base.filter(|path| !path.as_os_str().is_empty())
        .map(|path| path.join(APP_DISPLAY_NAME))
        .ok_or(AppError::MissingAppDataFolder)
}
