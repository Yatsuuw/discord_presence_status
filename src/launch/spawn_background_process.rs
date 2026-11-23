//! Start of a background copy of the application.

use std::process::Command;

use crate::error::{AppError, io_error_at};
use crate::launch::{PresenceHandover, background_arguments};

/// Starts a new copy of the running executable in background mode,
/// resuming `handover`. The calling process is expected to exit right after.
///
/// # Errors
///
/// Returns [`AppError::ExecutableLocation`] or [`AppError::Io`] when the
/// executable cannot be found or started.
pub fn spawn_background_process(handover: Option<&PresenceHandover>) -> Result<(), AppError> {
    let executable = std::env::current_exe().map_err(AppError::ExecutableLocation)?;
    Command::new(&executable)
        .args(background_arguments(handover))
        .spawn()
        .map(drop)
        .map_err(io_error_at(&executable))
}
