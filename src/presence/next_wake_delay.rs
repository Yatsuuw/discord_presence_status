//! Computation of the worker's next wake-up.

use std::time::Duration;

use crate::constants::{PRESENCE_REFRESH_INTERVAL, PRESENCE_RETRY_INTERVAL};
use crate::presence::WorkerState;

/// Returns how long the worker may sleep before acting on its own.
///
/// * `None`: nothing to do until the next command (no activity, or refused);
/// * retry interval: the activity is still waiting for Discord;
/// * refresh interval: the activity is displayed and only needs refreshing.
#[must_use]
pub fn next_wake_delay(state: &WorkerState) -> Option<Duration> {
    if state.desired.is_none() || state.rejected {
        None
    } else if state.pending {
        Some(PRESENCE_RETRY_INTERVAL)
    } else {
        Some(PRESENCE_REFRESH_INTERVAL)
    }
}
