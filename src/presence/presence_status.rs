//! Definition of [`PresenceStatus`], the state reported by the worker.

use crate::presence::PresenceError;

/// The state of the Discord presence, as reported to the user interface.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum PresenceStatus {
    /// No activity is displayed.
    #[default]
    Idle,
    /// The activity is displayed on the Discord profile.
    Active {
        /// Name of the displayed activity.
        activity_name: String,
        /// Unix time, in milliseconds, at which the timer started.
        started_at_millis: i64,
    },
    /// Discord is not reachable; the worker keeps trying.
    Waiting {
        /// Name of the activity waiting to be displayed.
        activity_name: String,
        /// Why Discord could not be reached.
        error: PresenceError,
    },
    /// Discord refused the activity; the worker stops trying until the next command.
    Rejected {
        /// Name of the refused activity.
        activity_name: String,
        /// The refusal, with the reason given by Discord.
        error: PresenceError,
    },
}
