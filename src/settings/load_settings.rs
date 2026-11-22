//! Reading of the settings file.

use std::io::ErrorKind;
use std::path::Path;

use crate::error::{AppError, io_error_at};
use crate::settings::Settings;

/// Reads the preferences stored at `path`.
///
/// A missing file is not an error: it simply means the defaults apply
/// (first launch).
///
/// # Errors
///
/// Returns [`AppError::Io`] when the file exists but cannot be read, and
/// [`AppError::SettingsParse`] when its content is invalid. Callers should
/// then fall back to [`Settings::default`].
pub fn load_settings(path: &Path) -> Result<Settings, AppError> {
    let content = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(Settings::default()),
        Err(error) => return Err(io_error_at(path)(error)),
    };
    toml::from_str(&content).map_err(|source| AppError::SettingsParse {
        path: path.to_path_buf(),
        source,
    })
}
