//! Selection of the texts of a language.

use crate::i18n::{ENGLISH, FRENCH, Language, Translations};

/// Returns every text of the interface in `language`.
#[must_use]
pub const fn translations_for(language: Language) -> &'static Translations {
    match language {
        Language::English => &ENGLISH,
        Language::French => &FRENCH,
    }
}
