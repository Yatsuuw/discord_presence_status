//! Current time as a Unix timestamp.

use std::time::{SystemTime, UNIX_EPOCH};

/// Returns the number of milliseconds elapsed since the Unix epoch.
///
/// Returns `0` if the system clock is set before 1970, which only happens on
/// badly configured machines and merely makes Discord's timer look wrong.
#[must_use]
pub fn current_unix_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .and_then(|elapsed| i64::try_from(elapsed.as_millis()).ok())
        .unwrap_or_default()
}
