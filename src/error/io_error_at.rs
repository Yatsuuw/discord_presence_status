//! Helper turning an [`std::io::Error`] into an [`AppError::Io`].

use std::path::Path;

use crate::error::AppError;

/// Returns a closure that wraps an I/O error with the path it happened on.
///
/// Designed for `map_err`, so file system calls stay one-liners:
///
/// ```
/// # use discord_presence_status::error::io_error_at;
/// # let path = std::path::Path::new("missing.txt");
/// let result = std::fs::read(path).map_err(io_error_at(path));
/// assert!(result.is_err());
/// ```
pub fn io_error_at(path: &Path) -> impl FnOnce(std::io::Error) -> AppError + '_ {
    move |source| AppError::Io {
        path: path.to_path_buf(),
        source,
    }
}
