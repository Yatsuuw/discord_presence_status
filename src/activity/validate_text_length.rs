//! Length check shared by every optional text field.

use crate::activity::{TextField, ValidationError};

/// Checks that an optional text has between `min` and `max` characters.
///
/// `None` is always accepted: optional fields may simply be left empty.
/// Length is counted in Unicode scalar values, not bytes, so accented letters
/// and emoji count as one character each.
///
/// # Errors
///
/// Returns [`ValidationError::InvalidLength`] when the text is out of bounds.
pub fn validate_text_length(
    field: TextField,
    value: Option<&str>,
    min: usize,
    max: usize,
) -> Result<(), ValidationError> {
    let Some(text) = value else {
        return Ok(());
    };
    let length = text.chars().count();
    if (min..=max).contains(&length) {
        Ok(())
    } else {
        Err(ValidationError::InvalidLength { field, min, max })
    }
}
