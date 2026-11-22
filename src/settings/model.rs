//! Definition of [`Settings`], the user preferences.

use serde::{Deserialize, Serialize};

use crate::i18n::Language;

/// The user preferences saved in `settings.toml`.
///
/// Missing fields take their default value, so files written by older
/// versions keep loading; unknown fields are ignored.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Settings {
    /// Language of the user interface.
    #[serde(default)]
    pub language: Language,
}
