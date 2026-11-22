//! Definition of [`ValidationError`], the reasons an activity is rejected.

use thiserror::Error;

/// Why an [`Activity`](crate::activity::Activity) cannot be saved or shown.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ValidationError {
    /// The name is empty.
    #[error("the activity name is required")]
    MissingName,

    /// The name only contains characters that cannot be used in a folder name.
    #[error("the activity name must contain characters usable in a folder name")]
    UnusableName,

    /// The Discord application ID is not a 17 to 20 digit number.
    #[error("the Discord application ID must be a number of 17 to 20 digits")]
    InvalidApplicationId,

    /// A text field is too short or too long for Discord.
    #[error("{field} must be between {min} and {max} characters long")]
    InvalidLength {
        /// Human-readable name of the field.
        field: &'static str,
        /// Minimum accepted length, in characters.
        min: usize,
        /// Maximum accepted length, in characters.
        max: usize,
    },

    /// The Discord image is neither an asset key nor an `https://` URL.
    #[error(
        "the Discord image must be an asset key or an https:// URL of at most {max} characters"
    )]
    InvalidImageReference {
        /// Maximum accepted length, in characters.
        max: usize,
    },
}
