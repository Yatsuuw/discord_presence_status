//! Definition of [`PresenceHandover`], the activity passed to a new process.

use std::path::PathBuf;

/// The displayed activity, handed over from one process to the next so the
/// Discord timer keeps running.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PresenceHandover {
    /// Folder of the displayed activity.
    pub folder: PathBuf,
    /// Unix time, in milliseconds, at which the Discord timer started.
    pub started_at_millis: i64,
}
