//! Definition of [`Language`], the languages of the interface.

use serde::{Deserialize, Serialize};

/// A language the user interface can be displayed in.
///
/// Stored in `settings.toml` by its code (`"en"`, `"fr"`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    /// English, the default language.
    #[default]
    #[serde(rename = "en")]
    English,
    /// French.
    #[serde(rename = "fr")]
    French,
}

impl Language {
    /// Every supported language, in the order they are offered to the user.
    pub const ALL: [Self; 2] = [Self::English, Self::French];
}
