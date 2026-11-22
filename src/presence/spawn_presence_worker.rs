//! Start-up of the presence worker thread.

use std::sync::mpsc;
use std::thread;

use crate::error::AppError;
use crate::presence::{PresenceHandle, RepaintNotifier, run_presence_worker};

/// Name given to the worker thread (visible in debuggers and crash reports).
const THREAD_NAME: &str = "presence-worker";

/// Starts the presence worker and returns the handle used to drive it.
///
/// `notify` is called every time the worker reports a new status.
///
/// # Errors
///
/// Returns [`AppError::WorkerSpawn`] when the thread cannot be created.
pub fn spawn_presence_worker(notify: RepaintNotifier) -> Result<PresenceHandle, AppError> {
    let (command_sender, command_receiver) = mpsc::channel();
    let (status_sender, status_receiver) = mpsc::channel();

    let thread = thread::Builder::new()
        .name(THREAD_NAME.to_owned())
        .spawn(move || run_presence_worker(&command_receiver, &status_sender, &notify))
        .map_err(AppError::WorkerSpawn)?;

    Ok(PresenceHandle {
        commands: command_sender,
        statuses: status_receiver,
        thread: Some(thread),
    })
}
