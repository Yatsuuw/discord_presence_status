//! Conversion of an [`ActivityKind`] into the Discord IPC type.

use discord_rich_presence::activity::ActivityType;

use crate::activity::ActivityKind;

/// Maps the application's activity kind to the Discord IPC activity type.
#[must_use]
pub const fn to_discord_activity_type(kind: ActivityKind) -> ActivityType {
    match kind {
        ActivityKind::Playing => ActivityType::Playing,
        ActivityKind::Listening => ActivityType::Listening,
        ActivityKind::Watching => ActivityType::Watching,
        ActivityKind::Competing => ActivityType::Competing,
    }
}
