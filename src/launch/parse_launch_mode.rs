//! Reading of the command-line arguments.

use std::ffi::OsString;
use std::path::PathBuf;

use crate::constants::{CLI_ACTIVITY_OPTION, CLI_BACKGROUND_FLAG, CLI_STARTED_AT_OPTION};
use crate::launch::{LaunchMode, PresenceHandover};

/// Determines the launch mode from the command-line arguments (without the
/// program name).
///
/// Anything unexpected falls back to a normal start: the user then simply
/// sees the window. A background start with an incomplete or invalid
/// activity starts without resuming it.
#[must_use]
pub fn parse_launch_mode(arguments: impl IntoIterator<Item = OsString>) -> LaunchMode {
    let mut background = false;
    let mut folder: Option<PathBuf> = None;
    let mut started_at_millis: Option<i64> = None;

    let mut arguments = arguments.into_iter();
    while let Some(argument) = arguments.next() {
        match argument.to_str() {
            Some(CLI_BACKGROUND_FLAG) => background = true,
            Some(CLI_ACTIVITY_OPTION) => folder = arguments.next().map(PathBuf::from),
            Some(CLI_STARTED_AT_OPTION) => {
                started_at_millis = arguments
                    .next()
                    .and_then(|value| value.to_str().and_then(|text| text.parse().ok()));
            }
            _ => {}
        }
    }

    if !background {
        return LaunchMode::Window;
    }
    let handover = folder
        .zip(started_at_millis)
        .map(|(folder, started_at_millis)| PresenceHandover {
            folder,
            started_at_millis,
        });
    LaunchMode::Background(handover)
}
