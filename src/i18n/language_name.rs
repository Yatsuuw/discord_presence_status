//! Names of the languages, as shown in the language selector.

use crate::i18n::Language;

/// Returns the name of `language` written in that language, so that anyone
/// can find their own language whatever the current one is.
#[must_use]
pub const fn language_name(language: Language) -> &'static str {
    match language {
        Language::English => "English",
        Language::French => "Français",
    }
}
