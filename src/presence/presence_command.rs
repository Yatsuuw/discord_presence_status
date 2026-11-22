//! Definition of [`PresenceCommand`], the messages sent to the worker.

use crate::presence::PresenceRequest;

/// An order sent by the user interface to the presence worker.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PresenceCommand {
    /// Display this activity (replacing the current one, if any).
    Show(PresenceRequest),
    /// Remove the activity from the Discord profile.
    Clear,
    /// Remove the activity and stop the worker thread.
    Shutdown,
}
