//! Definition of [`Activity`], the data saved in `activity.toml`.

use serde::{Deserialize, Serialize};

use crate::activity::ActivityKind;

/// A user-defined Discord activity.
///
/// It is stored as TOML in `user_activity/<folder>/activity.toml`, for example:
///
/// ```toml
/// name = "Rust programming"
/// application_id = "123456789012345678"
/// activity_type = "playing"
/// details = "Building a desktop application"
/// state = "Coffee number 3"
/// large_image = "https://example.com/rust.png"
/// large_text = "Ferris says hi"
/// local_image = "image.png"
/// ```
///
/// Optional fields are omitted from the file when empty. Unknown fields are
/// ignored, so files written by newer versions can still be read.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Activity {
    /// Name displayed by Discord (for example "Playing **Rust programming**").
    pub name: String,

    /// Identifier of the Discord application created in the Developer Portal.
    pub application_id: String,

    /// Verb Discord puts in front of the name.
    #[serde(default, rename = "activity_type")]
    pub kind: ActivityKind,

    /// First line under the name (the description).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,

    /// Second line under the name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// Image shown by Discord: a Rich Presence asset key or an `https://` URL.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub large_image: Option<String>,

    /// Tooltip displayed when hovering the Discord image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub large_text: Option<String>,

    /// File name of the local image stored next to `activity.toml`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_image: Option<String>,
}
