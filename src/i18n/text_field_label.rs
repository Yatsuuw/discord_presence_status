//! Translated names of the text fields subject to length limits.

use crate::activity::TextField;
use crate::i18n::Translations;

/// Returns the name of `field` as used at the start of a sentence.
#[must_use]
pub const fn text_field_label(field: TextField, texts: &Translations) -> &'static str {
    match field {
        TextField::Name => texts.length_field_name,
        TextField::Details => texts.length_field_details,
        TextField::State => texts.length_field_state,
        TextField::ImageTooltip => texts.length_field_image_tooltip,
    }
}
