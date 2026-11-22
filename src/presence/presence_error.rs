//! Definition of [`PresenceError`], the failures of a Discord exchange.

use thiserror::Error;

/// Why an activity could not be displayed.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PresenceError {
    /// Discord is not running or the connection was lost. Worth retrying.
    #[error("Discord is not reachable ({0}). Is the Discord desktop app running?")]
    Unavailable(String),
    /// Discord answered with an error. Retrying the same request is pointless.
    #[error("Discord refused the activity: {0}")]
    Rejected(String),
}
