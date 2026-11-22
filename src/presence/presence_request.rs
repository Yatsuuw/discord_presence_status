//! Definition of [`PresenceRequest`], what the worker must display.

use crate::activity::Activity;

/// An activity to display, together with the start of its Discord timer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PresenceRequest {
    /// The activity to display.
    pub activity: Activity,
    /// Unix time, in milliseconds, at which Discord's elapsed timer starts.
    ///
    /// It is fixed when the user starts the activity, so reconnections and
    /// edits keep the timer running instead of resetting it.
    pub started_at_millis: i64,
}
