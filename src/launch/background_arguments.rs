//! Command-line arguments of a background start.

use std::ffi::OsString;

use crate::constants::{CLI_ACTIVITY_OPTION, CLI_BACKGROUND_FLAG, CLI_STARTED_AT_OPTION};
use crate::launch::PresenceHandover;

/// Returns the arguments starting the application in background mode,
/// resuming `handover` when there is one.
///
/// [`parse_launch_mode`](crate::launch::parse_launch_mode) reads them back.
#[must_use]
pub fn background_arguments(handover: Option<&PresenceHandover>) -> Vec<OsString> {
    let mut arguments = vec![OsString::from(CLI_BACKGROUND_FLAG)];
    if let Some(handover) = handover {
        arguments.extend([
            OsString::from(CLI_ACTIVITY_OPTION),
            handover.folder.clone().into_os_string(),
            OsString::from(CLI_STARTED_AT_OPTION),
            OsString::from(handover.started_at_millis.to_string()),
        ]);
    }
    arguments
}
