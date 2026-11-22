//! Location of the running executable.

use std::path::PathBuf;

use crate::error::AppError;

/// Returns the folder containing the running executable.
///
/// # Errors
///
/// Returns [`AppError::ExecutableLocation`] when the operating system cannot
/// tell where the executable is.
pub fn executable_directory() -> Result<PathBuf, AppError> {
    let executable = std::env::current_exe().map_err(AppError::ExecutableLocation)?;
    executable
        .parent()
        .map(std::path::Path::to_path_buf)
        .ok_or_else(|| {
            AppError::ExecutableLocation(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "the executable path has no parent folder",
            ))
        })
}
