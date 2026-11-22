//! Validation of a complete [`Activity`].

use crate::activity::{
    Activity, TextField, ValidationError, is_valid_application_id, sanitize_folder_name,
    validate_image_reference, validate_text_length,
};
use crate::constants::{MAX_PRESENCE_TEXT_LENGTH, MIN_PRESENCE_TEXT_LENGTH};

/// Checks that an activity can be saved and accepted by Discord.
///
/// Returns the first rule that is broken, in the order the fields appear in
/// the editor, so the user can fix them from top to bottom.
///
/// # Errors
///
/// Returns the [`ValidationError`] describing the first invalid field.
pub fn validate_activity(activity: &Activity) -> Result<(), ValidationError> {
    let name = activity.name.trim();
    if name.is_empty() {
        return Err(ValidationError::MissingName);
    }
    validate_text_length(
        TextField::Name,
        Some(name),
        MIN_PRESENCE_TEXT_LENGTH,
        MAX_PRESENCE_TEXT_LENGTH,
    )?;
    if sanitize_folder_name(name).is_empty() {
        return Err(ValidationError::UnusableName);
    }

    if !is_valid_application_id(&activity.application_id) {
        return Err(ValidationError::InvalidApplicationId);
    }

    let text_fields = [
        (TextField::Details, activity.details.as_deref()),
        (TextField::State, activity.state.as_deref()),
        (TextField::ImageTooltip, activity.large_text.as_deref()),
    ];
    for (field, value) in text_fields {
        validate_text_length(
            field,
            value,
            MIN_PRESENCE_TEXT_LENGTH,
            MAX_PRESENCE_TEXT_LENGTH,
        )?;
    }

    validate_image_reference(activity.large_image.as_deref())
}
