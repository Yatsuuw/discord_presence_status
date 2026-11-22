//! Definition of [`ActivePresence`], the activity sent to Discord.

use std::path::PathBuf;

/// The activity the user asked Discord to display.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivePresence {
    /// Folder of the displayed activity.
    pub folder: PathBuf,
    /// Unix time, in milliseconds, at which the Discord timer started.
    pub started_at_millis: i64,
}
