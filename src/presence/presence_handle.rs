//! Definition of [`PresenceHandle`], the UI side of the worker channels.

use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;

use crate::presence::{PresenceCommand, PresenceStatus};

/// Everything the user interface needs to drive the presence worker.
#[derive(Debug)]
pub struct PresenceHandle {
    /// Channel used to send commands to the worker.
    pub commands: Sender<PresenceCommand>,
    /// Channel on which the worker reports status changes.
    pub statuses: Receiver<PresenceStatus>,
    /// The worker thread, joined on shutdown.
    pub thread: Option<JoinHandle<()>>,
}
