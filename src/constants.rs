//! Application-wide constant values.
//!
//! Keeping every "magic value" in one place makes it obvious what can be
//! tuned and guarantees that all modules agree on the same names.

use std::time::Duration;

/// Human-readable application name (window title, data folder name).
pub const APP_DISPLAY_NAME: &str = "Discord Presence Status";

/// Name of the folder, inside the data root, that holds one sub-folder per activity.
pub const ACTIVITIES_FOLDER_NAME: &str = "user_activity";

/// Name of the file describing an activity inside its own folder.
pub const ACTIVITY_FILE_NAME: &str = "activity.toml";

/// File stem used for the local image copied into an activity folder.
pub const ACTIVITY_IMAGE_STEM: &str = "image";

/// Image file extensions accepted for the local activity image (lower case).
pub const SUPPORTED_IMAGE_EXTENSIONS: &[&str] = &["png", "jpg", "jpeg", "webp"];

/// Largest local image accepted, in bytes.
pub const MAX_IMAGE_SIZE_BYTES: u64 = 8 * 1024 * 1024;

/// Longest folder name generated from an activity name, in characters.
pub const MAX_FOLDER_NAME_LENGTH: usize = 64;

/// Shortest text Discord accepts for a presence field, in characters.
pub const MIN_PRESENCE_TEXT_LENGTH: usize = 2;

/// Longest text Discord accepts for a presence field, in characters.
pub const MAX_PRESENCE_TEXT_LENGTH: usize = 128;

/// Longest Discord image reference (asset key or URL), in characters.
pub const MAX_IMAGE_REFERENCE_LENGTH: usize = 256;

/// Delay before trying again when the Discord client is not reachable.
pub const PRESENCE_RETRY_INTERVAL: Duration = Duration::from_secs(5);

/// Delay between two refreshes of an active presence.
///
/// Refreshing regularly lets the worker notice that Discord was restarted and
/// restore the presence automatically. It stays well below Discord's rate
/// limit of five updates per twenty seconds.
pub const PRESENCE_REFRESH_INTERVAL: Duration = Duration::from_secs(20);
