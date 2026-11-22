//! Validation of the image Discord displays.

use crate::activity::ValidationError;
use crate::constants::MAX_IMAGE_REFERENCE_LENGTH;

/// Checks the Discord image reference of an activity.
///
/// Discord accepts either the key of an asset uploaded in the Developer
/// Portal (for example `rust_logo`) or a public `https://` URL. `None` is
/// accepted because the image is optional.
///
/// # Errors
///
/// Returns [`ValidationError::InvalidImageReference`] when the reference is
/// too long, contains whitespace, or uses a scheme other than `https://`.
pub fn validate_image_reference(reference: Option<&str>) -> Result<(), ValidationError> {
    let Some(reference) = reference else {
        return Ok(());
    };
    let error = ValidationError::InvalidImageReference {
        max: MAX_IMAGE_REFERENCE_LENGTH,
    };

    let too_long = reference.chars().count() > MAX_IMAGE_REFERENCE_LENGTH;
    let has_whitespace = reference.chars().any(char::is_whitespace);
    let has_bad_scheme = reference.contains("://") && !reference.starts_with("https://");

    if reference.is_empty() || too_long || has_whitespace || has_bad_scheme {
        Err(error)
    } else {
        Ok(())
    }
}
