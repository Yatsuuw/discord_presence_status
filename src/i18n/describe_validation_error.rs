//! Translation of validation errors.

use crate::activity::ValidationError;
use crate::i18n::{Translations, fill_placeholders, text_field_label};

/// Explains `error` to the user in the interface language.
#[must_use]
pub fn describe_validation_error(error: &ValidationError, texts: &Translations) -> String {
    match error {
        ValidationError::MissingName => texts.validation_missing_name.to_owned(),
        ValidationError::UnusableName => texts.validation_unusable_name.to_owned(),
        ValidationError::InvalidApplicationId => texts.validation_invalid_application_id.to_owned(),
        ValidationError::InvalidLength { field, min, max } => fill_placeholders(
            texts.validation_invalid_length,
            &[
                ("field", text_field_label(*field, texts)),
                ("min", &min.to_string()),
                ("max", &max.to_string()),
            ],
        ),
        ValidationError::InvalidImageReference { max } => {
            fill_placeholders(texts.validation_invalid_image, &[("max", &max.to_string())])
        }
    }
}
