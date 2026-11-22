//! Translated labels of activity kinds.

use crate::activity::ActivityKind;
use crate::i18n::Translations;

/// Returns the verb Discord displays for `kind`, in the interface language.
///
/// Discord itself shows the verb in the language of each viewer; this label
/// only helps the user choose.
#[must_use]
pub const fn activity_kind_label(kind: ActivityKind, texts: &Translations) -> &'static str {
    match kind {
        ActivityKind::Playing => texts.kind_playing,
        ActivityKind::Listening => texts.kind_listening,
        ActivityKind::Watching => texts.kind_watching,
        ActivityKind::Competing => texts.kind_competing,
    }
}
