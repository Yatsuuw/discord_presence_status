//! Clean stop of the presence worker thread.

use crate::presence::{PresenceCommand, PresenceHandle};

/// Clears the Discord presence and waits for the worker thread to finish.
///
/// Safe to call several times: later calls do nothing.
pub fn shutdown_presence_worker(handle: &mut PresenceHandle) {
    let _ = handle.commands.send(PresenceCommand::Shutdown);
    if let Some(thread) = handle.thread.take() {
        let _ = thread.join();
    }
}
